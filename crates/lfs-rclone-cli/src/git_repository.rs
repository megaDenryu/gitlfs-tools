//! 対象Gitリポジトリの`--local`設定を読み書きする外部境界。`git`を子プロセスとして
//! 起動し、リポジトリの検出は`git`自身のディレクトリ探索（起動したディレクトリから
//! 親をたどる`.git`探索）に委ねる（コード分割規約.md 1節「外部境界」）。
//!
//! `git`を動かす場所は`Gitコマンドの実行場所`が保持する。`install`は現在地のまま動き、
//! `clone`は複製した先のディレクトリを指定する。
//!
//! 注意: 設定の書き込みは常に`--local`だけを使い、`--global`は使わない
//! （CLAUDE.md「全Gitリポジトリへ無条件に適用してはならない」）。

use std::process::{Command, Output};

use crate::command_error::コマンド実行エラー;
use crate::git_command_directory::Gitコマンドの実行場所;
use crate::git_hook_directory::Gitフック置き場;
use crate::work_tree_root::作業ツリールート;

/// 指定した実行場所から到達できるGitリポジトリを表す。パスの綴りをこの型の
/// メソッドへ閉じ、`git`コマンドの引数組み立てを呼び出し側へ散らさない
/// （グローバルCLAUDE.md「役割の型は自分の配置を知る」）。
pub(crate) struct Gitリポジトリ {
    実行場所: Gitコマンドの実行場所,
}

/// `ローカル設定を書き込んで結果を返す`が返す、書き込み前の状態の分類。
pub(crate) enum 設定書き込み結果 {
    新規,
    変更なし,
    更新前 { 旧値: String },
}

impl Gitリポジトリ {
    /// 現在の作業ディレクトリがGitリポジトリの作業ツリーの中であることを確かめる。
    pub(crate) fn 現在地から検出する() -> Result<Self, コマンド実行エラー> {
        Self::実行場所を指定して検出する(Gitコマンドの実行場所::現在地)
    }

    /// 指定したディレクトリがGitリポジトリの作業ツリーの中であることを確かめる。
    pub(crate) fn 実行場所を指定して検出する(実行場所: Gitコマンドの実行場所) -> Result<Self, コマンド実行エラー> {
        let リポジトリ = Self { 実行場所 };
        let 出力 = リポジトリ.gitの出力を得る(&["rev-parse", "--is-inside-work-tree"])?;
        if 出力.status.success() { Ok(リポジトリ) } else { Err(コマンド実行エラー::Gitリポジトリ外) }
    }

    /// 作業ツリーのルート。配置の導出は`作業ツリールート`のメソッドへ閉じる。
    pub(crate) fn 作業ツリーのルート(&self) -> Result<作業ツリールート, コマンド実行エラー> {
        let 標準出力 = self.成功した出力の標準出力を得る(&["rev-parse", "--show-toplevel"])?;
        Ok(作業ツリールート::生成する(標準出力.into()))
    }

    /// Gitがフックを探すディレクトリ。`core.hooksPath`が設定されていればそちらを指すため、
    /// `.git/hooks`を直に綴らず`git rev-parse --git-path hooks`に決めさせる。
    pub(crate) fn フック置き場を取得する(&self) -> Result<Gitフック置き場, コマンド実行エラー> {
        let 標準出力 = self.成功した出力の標準出力を得る(&["rev-parse", "--git-path", "hooks"])?;
        Ok(Gitフック置き場::生成する(標準出力.into()))
    }

    /// `--local`設定を取得する。未設定またはコマンド失敗は区別せず`None`にする
    /// （doctorの「登録されているか」判定にはどちらも同じ意味であるため）。
    pub(crate) fn ローカル設定を取得する(&self, キー: &str) -> Option<String> {
        let 出力 = self.gitの出力を得る(&["config", "--local", "--get", キー]).ok()?;
        if !出力.status.success() {
            return None;
        }
        Some(String::from_utf8_lossy(&出力.stdout).trim().to_owned())
    }

    /// `--local`設定を書き込み、書き込み前の状態（新規・変更なし・更新前の旧値）を返す。
    /// 既存値の取得と書き込みの両方が`Gitリポジトリ`自身の操作であり、呼び出し側へ
    /// `Gitリポジトリ`を引数として渡させない（グローバルCLAUDE.md「操作の所有者を
    /// 外部引数にしない」）。
    pub(crate) fn ローカル設定を書き込んで結果を返す(&self, キー: &str, 値: &str) -> Result<設定書き込み結果, コマンド実行エラー> {
        let 既存値 = self.ローカル設定を取得する(キー);
        self.ローカル設定を書き込む(キー, 値)?;
        Ok(match 既存値 {
            Some(既存) if 既存 == 値 => 設定書き込み結果::変更なし,
            Some(旧値) => 設定書き込み結果::更新前 { 旧値 },
            None => 設定書き込み結果::新規,
        })
    }

    fn ローカル設定を書き込む(&self, キー: &str, 値: &str) -> Result<(), コマンド実行エラー> {
        let 出力 = self.gitの出力を得る(&["config", "--local", キー, 値])?;
        if 出力.status.success() {
            Ok(())
        } else {
            Err(コマンド実行エラー::Git設定書き込み失敗 {
                キー: キー.to_owned(),
                説明: String::from_utf8_lossy(&出力.stderr).trim().to_owned(),
            })
        }
    }

    /// 問い合わせ系の`git`を実行し、失敗をGitリポジトリ外として扱ったうえで標準出力を返す。
    fn 成功した出力の標準出力を得る(&self, 引数: &[&str]) -> Result<String, コマンド実行エラー> {
        let 出力 = self.gitの出力を得る(引数)?;
        if !出力.status.success() {
            return Err(コマンド実行エラー::Gitリポジトリ外);
        }
        Ok(String::from_utf8_lossy(&出力.stdout).trim().to_owned())
    }

    fn gitの出力を得る(&self, 引数: &[&str]) -> Result<Output, コマンド実行エラー> {
        let mut コマンド = Command::new("git");
        コマンド.args(引数);
        self.実行場所.コマンドへ適用する(&mut コマンド);
        コマンド.output().map_err(|エラー| コマンド実行エラー::Gitコマンド起動失敗 { 説明: エラー.to_string() })
    }
}
