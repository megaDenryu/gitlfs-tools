//! `git lfs ls-files`を子プロセスとして起動し、Git LFSが参照するファイルの一覧を得る
//! 外部境界（コード分割規約.md「外部境界」）。
//!
//! 正確なバイト数を`--json`から得る。`-s`が出すサイズは人向けに丸めた表示（`5.0 KB`）で
//! あり突き合わせに使えず、pointerのblobを1件ずつ読む形は数千ファイルで子プロセスの起動が
//! 同じ数だけ要る。`--json`は1回の起動で`size`をバイト単位の整数として返す
//! （git-lfs 3.5.1で実測）。

use std::process::Command;

use gitlfs_tools_domain::オブジェクト識別子;
use gitlfs_tools_transfer::点検対象オブジェクト;

use crate::command_error::コマンド実行エラー;
use crate::git_lfs_ls_files_json::LsFiles出力;
use crate::object_check_scope::点検範囲;
use crate::tracked_lfs_file::GitLFS追跡ファイル;

/// Git LFSが参照するファイルの一覧。パスと識別子の対応を保持し、点検の出力で欠落した
/// オブジェクトがどのファイルの実体かを引けるようにする。
pub(crate) struct GitLFS追跡ファイル一覧(Vec<GitLFS追跡ファイル>);

impl GitLFS追跡ファイル一覧 {
    /// 現在の作業ディレクトリのリポジトリへ問い合わせる。
    pub(crate) fn 作業ディレクトリへ問い合わせる(範囲: 点検範囲) -> Result<Self, コマンド実行エラー> {
        let 出力 = Command::new("git")
            .args(範囲.ls_files引数())
            .output()
            .map_err(|エラー| コマンド実行エラー::Gitコマンド起動失敗 { 説明: エラー.to_string() })?;
        if !出力.status.success() {
            return Err(コマンド実行エラー::GitLFSの一覧取得失敗 {
                説明: String::from_utf8_lossy(&出力.stderr).trim().to_owned(),
            });
        }

        let 解析結果: LsFiles出力 = serde_json::from_slice(&出力.stdout).map_err(|エラー| {
            コマンド実行エラー::GitLFSの一覧取得失敗 { 説明: format!("ls-filesの出力を解析できませんでした: {エラー}") }
        })?;

        let mut 一覧 = Vec::new();
        for 要素 in 解析結果.files.unwrap_or_default() {
            let ファイル = GitLFS追跡ファイル::生成する(要素.name, &要素.oid, 要素.size)
                .map_err(|エラー| コマンド実行エラー::GitLFSの一覧取得失敗 { 説明: エラー.to_string() })?;
            一覧.push(ファイル);
        }
        Ok(Self(一覧))
    }

    pub(crate) fn 点検対象一覧へ変換する(&self) -> Vec<点検対象オブジェクト> {
        self.0.iter().map(GitLFS追跡ファイル::点検対象オブジェクトへ変換する).collect()
    }

    /// この識別子を実体として参照しているファイルのパス。同じ内容のファイルが複数ある場合は
    /// 複数返る。
    pub(crate) fn 識別子を参照するパス一覧(&self, 識別子: &オブジェクト識別子) -> Vec<&str> {
        self.0.iter().filter(|ファイル| ファイル.識別子() == 識別子).map(GitLFS追跡ファイル::パス).collect()
    }
}
