//! `.large-assets.toml`の正常解析・schema版拒否・未知キー拒否のテスト。

use std::io;

use gitlfs_tools_config::{プロジェクト設定の場所, 設定エラー};

fn プロジェクト設定ディレクトリを作る(内容: &str) -> io::Result<tempfile::TempDir> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    std::fs::write(作業ディレクトリ.path().join(".large-assets.toml"), 内容)?;
    Ok(作業ディレクトリ)
}

#[test]
fn 正常なプロジェクト設定を解析する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = プロジェクト設定ディレクトリを作る("schema_version = 1\nprofile = \"personal-large-assets\"\n")?;

    let プロジェクト設定 = プロジェクト設定の場所::探索する(作業ディレクトリ.path())?.読み込む()?;

    assert_eq!(プロジェクト設定.プロファイル().文字列表現(), "personal-large-assets");
    Ok(())
}

#[test]
fn 未対応のschema版を拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = プロジェクト設定ディレクトリを作る("schema_version = 2\nprofile = \"x\"\n")?;

    let 結果 = プロジェクト設定の場所::探索する(作業ディレクトリ.path())?.読み込む();

    assert!(matches!(
        結果,
        Err(設定エラー::未対応スキーマ版 { 受信した版: 2, 受理できる版: 1 })
    ));
    Ok(())
}

#[test]
fn 未知キーを含むプロジェクト設定を拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\nprofile = \"x\"\nrclone_remote = \"leaked-remote\"\n";
    let 作業ディレクトリ = プロジェクト設定ディレクトリを作る(内容)?;

    let 結果 = プロジェクト設定の場所::探索する(作業ディレクトリ.path())?.読み込む();

    assert!(matches!(結果, Err(設定エラー::解析失敗 { .. })));
    Ok(())
}
