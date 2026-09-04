//! テスト用の隔離Gitリポジトリを作る。実ユーザーのGit設定を汚染しないよう、常に
//! `--local`検査対象の一時ディレクトリの中だけへ`git init`する。

use std::path::Path;
use std::process::Command;

pub fn 初期化する(ディレクトリ: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let 終了状態 = Command::new("git").args(["init", "-q"]).current_dir(ディレクトリ).status()?;
    if 終了状態.success() { Ok(()) } else { Err("git initに失敗しました".into()) }
}

/// 実体の送信を止める旧方式の`pre-push`フックを置く。`git lfs install --local`が
/// `Hook already exists: pre-push`で止まる状態を、実リポジトリを使わずに再現する。
pub fn 送信を止める旧pre_pushフックを置く(ディレクトリ: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let 置き場 = ディレクトリ.join(".git").join("hooks");
    std::fs::create_dir_all(&置き場)?;
    let 本文 = "#!/bin/sh\nexport GIT_LFS_SKIP_PUSH=1\ngit lfs pre-push \"$@\"\n";
    std::fs::write(置き場.join("pre-push"), 本文)?;
    Ok(())
}

/// `git lfs install --local`を実行し、このリポジトリのフィルターを有効化する。
pub fn lfsを有効化する(ディレクトリ: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let 終了状態 = Command::new("git").args(["lfs", "install", "--local"]).current_dir(ディレクトリ).status()?;
    if 終了状態.success() { Ok(()) } else { Err("git lfs install --localに失敗しました".into()) }
}
