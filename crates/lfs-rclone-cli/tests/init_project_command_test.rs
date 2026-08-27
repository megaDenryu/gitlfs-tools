//! `init-project`サブコマンド: リポジトリのルートへ`.large-assets.toml`の雛形を作ること、
//! 既存ファイルを無言で上書きしないこと、Gitリポジトリの外では失敗することを確かめる。

mod common;

use std::fs;

#[test]
fn init_projectは雛形をリポジトリのルートへ作る() -> Result<(), Box<dyn std::error::Error>> {
    let リポジトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(リポジトリ.path())?;
    let サブディレクトリ = リポジトリ.path().join("nested");
    fs::create_dir(&サブディレクトリ)?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(&サブディレクトリ, &["init-project", "--profile", "my-profile"], &[])?;
    assert!(結果.成功したか, "init-projectは成功するべき: stderr={}", 結果.標準エラー出力);

    let 配置先 = リポジトリ.path().join(".large-assets.toml");
    let 本文 = fs::read_to_string(&配置先)?;
    assert_eq!(本文, "schema_version = 1\nprofile = \"my-profile\"\n");
    Ok(())
}

#[test]
fn init_projectは既存ファイルを無言で上書きしない() -> Result<(), Box<dyn std::error::Error>> {
    let リポジトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(リポジトリ.path())?;
    let 配置先 = リポジトリ.path().join(".large-assets.toml");
    fs::write(&配置先, "schema_version = 1\nprofile = \"既存\"\n")?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(リポジトリ.path(), &["init-project", "--profile", "new-profile"], &[])?;
    assert!(!結果.成功したか, "既存ファイルがある場合は失敗するべき");

    let 本文 = fs::read_to_string(&配置先)?;
    assert_eq!(本文, "schema_version = 1\nprofile = \"既存\"\n", "既存ファイルの内容は変わらないべき");
    Ok(())
}

#[test]
fn init_projectはgitリポジトリの外では失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 結果 = common::cli_invocation::サブコマンドを実行する(ディレクトリ.path(), &["init-project", "--profile", "x"], &[])?;
    assert!(!結果.成功したか, "Gitリポジトリの外でのinit-projectは失敗するべき");
    Ok(())
}
