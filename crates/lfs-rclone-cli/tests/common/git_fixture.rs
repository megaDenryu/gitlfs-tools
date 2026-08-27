//! テスト用の隔離Gitリポジトリを作る。実ユーザーのGit設定を汚染しないよう、常に
//! `--local`検査対象の一時ディレクトリの中だけへ`git init`する。

use std::path::Path;
use std::process::Command;

pub fn 初期化する(ディレクトリ: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let 終了状態 = Command::new("git").args(["init", "-q"]).current_dir(ディレクトリ).status()?;
    if 終了状態.success() { Ok(()) } else { Err("git initに失敗しました".into()) }
}

/// `git lfs install --local`を実行し、このリポジトリのフィルターを有効化する。
pub fn lfsを有効化する(ディレクトリ: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let 終了状態 = Command::new("git").args(["lfs", "install", "--local"]).current_dir(ディレクトリ).status()?;
    if 終了状態.success() { Ok(()) } else { Err("git lfs install --localに失敗しました".into()) }
}
