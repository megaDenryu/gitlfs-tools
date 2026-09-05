//! `release`がgitへ問い合わせ・指示する操作一式。gitを子プロセスとして起動する外部境界であり、
//! ここでは判断をせず、gitが答えた事実だけを返す。発行してよいかどうかの判断は
//! `unmet_precondition.rs`が持つ。

use std::path::PathBuf;
use std::process::Command;

use crate::release::release_tag::リリースタグ名;

/// gitの操作対象になる作業ツリー。どのディレクトリに対して起動するかを保持する。
pub struct Git作業ツリー(PathBuf);

impl Git作業ツリー {
    pub fn カレントディレクトリを対象にする() -> Result<Self, String> {
        std::env::current_dir()
            .map(Self)
            .map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))
    }

    pub fn 未コミットの変更が残っているか(&self) -> Result<bool, String> {
        Ok(!self.標準出力を取り出す(&["status", "--porcelain"])?.trim().is_empty())
    }

    pub fn 現在のブランチ名(&self) -> Result<String, String> {
        Ok(self.標準出力を取り出す(&["rev-parse", "--abbrev-ref", "HEAD"])?.trim().to_owned())
    }

    pub fn 同じ名前のタグが既にあるか(&self, タグ名: &リリースタグ名) -> Result<bool, String> {
        let 出力 = self.標準出力を取り出す(&["tag", "--list", タグ名.タグ名の文字列()])?;
        Ok(!出力.trim().is_empty())
    }

    pub fn タグを作る(&self, タグ名: &リリースタグ名) -> Result<(), String> {
        self.終了状態だけを確かめる(&["tag", タグ名.タグ名の文字列()])
    }

    pub fn タグをoriginへ送る(&self, タグ名: &リリースタグ名) -> Result<(), String> {
        self.終了状態だけを確かめる(&["push", "origin", タグ名.タグ名の文字列()])
    }

    fn 標準出力を取り出す(&self, 引数: &[&str]) -> Result<String, String> {
        let 出力 = Command::new("git")
            .current_dir(&self.0)
            .args(引数)
            .output()
            .map_err(|失敗| format!("gitを起動できなかった: {失敗}"))?;
        if !出力.status.success() {
            return Err(失敗の説明を組み立てる(引数, &String::from_utf8_lossy(&出力.stderr)));
        }
        Ok(String::from_utf8_lossy(&出力.stdout).into_owned())
    }

    fn 終了状態だけを確かめる(&self, 引数: &[&str]) -> Result<(), String> {
        let 出力 = Command::new("git")
            .current_dir(&self.0)
            .args(引数)
            .output()
            .map_err(|失敗| format!("gitを起動できなかった: {失敗}"))?;
        if 出力.status.success() {
            return Ok(());
        }
        Err(失敗の説明を組み立てる(引数, &String::from_utf8_lossy(&出力.stderr)))
    }
}

fn 失敗の説明を組み立てる(引数: &[&str], 標準エラー出力: &str) -> String {
    format!("git {} が失敗した: {}", 引数.join(" "), 標準エラー出力.trim())
}
