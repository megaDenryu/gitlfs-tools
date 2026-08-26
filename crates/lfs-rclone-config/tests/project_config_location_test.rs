//! `.large-assets.toml`の親ディレクトリ探索と、未検出の区別のテスト。

use lfs_rclone_config::{プロジェクト設定の場所, 設定エラー};

#[test]
fn 親ディレクトリをさかのぼって設定ファイルを見つける() -> Result<(), Box<dyn std::error::Error>> {
    let リポジトリルート = tempfile::tempdir()?;
    let 設定ファイルパス = リポジトリルート.path().join(".large-assets.toml");
    std::fs::write(&設定ファイルパス, "schema_version = 1\nprofile = \"personal-large-assets\"\n")?;

    let 深いサブディレクトリ = リポジトリルート.path().join("a").join("b").join("c");
    std::fs::create_dir_all(&深いサブディレクトリ)?;

    let 場所 = プロジェクト設定の場所::探索する(&深いサブディレクトリ)?;

    assert_eq!(場所.パス(), 設定ファイルパス);
    Ok(())
}

#[test]
fn 設定ファイルが存在しない場合は未検出として区別される() -> Result<(), Box<dyn std::error::Error>> {
    let 空のディレクトリ = tempfile::tempdir()?;

    let 結果 = プロジェクト設定の場所::探索する(空のディレクトリ.path());

    assert!(matches!(結果, Err(設定エラー::プロジェクト設定未検出)));
    Ok(())
}
