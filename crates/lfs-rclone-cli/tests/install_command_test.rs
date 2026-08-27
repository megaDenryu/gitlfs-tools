//! `install`サブコマンド: 対象リポジトリだけへ`--local`のcustom transfer設定を登録し、
//! グローバル設定を変更しないこと、Gitリポジトリの外では失敗すること、再実行しても
//! 同じ値へ収束することを確かめる。実rcloneは不要（Git設定の読み書きだけを検査する）。

mod common;

use std::fs;
use std::process::Command;

#[test]
fn installはローカル設定だけを登録しグローバル設定を変更しない() -> Result<(), Box<dyn std::error::Error>> {
    let リポジトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(リポジトリ.path())?;

    let 隔離グローバル設定 = tempfile::tempdir()?;
    let グローバル設定ファイル = 隔離グローバル設定.path().join("gitconfig");
    fs::write(&グローバル設定ファイル, "")?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(
        リポジトリ.path(),
        &["install"],
        &[("GIT_CONFIG_GLOBAL", グローバル設定ファイル.as_os_str())],
    )?;
    assert!(結果.成功したか, "installは成功するべき: stderr={}", 結果.標準エラー出力);

    let パス値 = ローカル設定を読む(リポジトリ.path(), "lfs.customtransfer.rclone-storage.path")?;
    assert!(!パス値.trim().is_empty(), "実行ファイルパスが登録されているべき");
    assert_eq!(ローカル設定を読む(リポジトリ.path(), "lfs.customtransfer.rclone-storage.concurrent")?.trim(), "true");
    assert_eq!(ローカル設定を読む(リポジトリ.path(), "lfs.customtransfer.rclone-storage.direction")?.trim(), "both");
    assert_eq!(ローカル設定を読む(リポジトリ.path(), "lfs.standalonetransferagent")?.trim(), "rclone-storage");

    let グローバル設定の内容 = fs::read_to_string(&グローバル設定ファイル)?;
    assert!(グローバル設定の内容.trim().is_empty(), "グローバル設定は変更されていないべき: {グローバル設定の内容}");
    Ok(())
}

#[test]
fn installはgitリポジトリの外では失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 結果 = common::cli_invocation::サブコマンドを実行する(ディレクトリ.path(), &["install"], &[])?;
    assert!(!結果.成功したか, "Gitリポジトリの外でのinstallは失敗するべき");
    assert!(結果.標準エラー出力.contains("Git"), "失敗理由にGitへの言及があるべき: {}", 結果.標準エラー出力);
    Ok(())
}

#[test]
fn installを再実行しても同じ値へ収束する() -> Result<(), Box<dyn std::error::Error>> {
    let リポジトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(リポジトリ.path())?;

    let 初回 = common::cli_invocation::サブコマンドを実行する(リポジトリ.path(), &["install"], &[])?;
    assert!(初回.成功したか);
    let 再実行 = common::cli_invocation::サブコマンドを実行する(リポジトリ.path(), &["install"], &[])?;
    assert!(再実行.成功したか);

    assert_eq!(
        ローカル設定を読む(リポジトリ.path(), "lfs.customtransfer.rclone-storage.direction")?,
        "both\n"
    );
    Ok(())
}

fn ローカル設定を読む(リポジトリ: &std::path::Path, キー: &str) -> Result<String, Box<dyn std::error::Error>> {
    let 出力 = Command::new("git").args(["config", "--local", "--get", キー]).current_dir(リポジトリ).output()?;
    Ok(String::from_utf8_lossy(&出力.stdout).into_owned())
}
