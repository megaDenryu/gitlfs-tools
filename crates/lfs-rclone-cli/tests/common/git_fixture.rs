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

/// `git lfs track`でパターンを`.gitattributes`へ登録する。
pub fn 追跡パターンを登録する(ディレクトリ: &Path, パターン: &str) -> Result<(), Box<dyn std::error::Error>> {
    let 終了状態 = Command::new("git").args(["lfs", "track", パターン]).current_dir(ディレクトリ).status()?;
    if 終了状態.success() { Ok(()) } else { Err("git lfs trackに失敗しました".into()) }
}

/// 内容を書いたファイルをコミットする。コミット者の情報はこの呼び出しの中だけで与え、
/// 実ユーザーのGit設定を読まない。
pub fn ファイルを追加してコミットする(
    ディレクトリ: &Path,
    ファイル名: &str,
    内容: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(ディレクトリ.join(ファイル名), 内容)?;
    実行する(ディレクトリ, &["add", "-A"])?;
    コミットする(ディレクトリ, &format!("add {ファイル名}"))
}

/// 追跡中のファイルを削除してコミットする。過去の版だけに残る実体を作るために使う。
pub fn ファイルを削除してコミットする(ディレクトリ: &Path, ファイル名: &str) -> Result<(), Box<dyn std::error::Error>> {
    実行する(ディレクトリ, &["rm", "-q", ファイル名])?;
    コミットする(ディレクトリ, &format!("remove {ファイル名}"))
}

fn コミットする(ディレクトリ: &Path, メッセージ: &str) -> Result<(), Box<dyn std::error::Error>> {
    実行する(ディレクトリ, &["-c", "user.email=test@example.invalid", "-c", "user.name=test", "commit", "-q", "-m", メッセージ])
}

fn 実行する(ディレクトリ: &Path, 引数: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let 終了状態 = Command::new("git").args(引数).current_dir(ディレクトリ).status()?;
    if 終了状態.success() { Ok(()) } else { Err(format!("gitコマンドに失敗しました: {引数:?}").into()) }
}
