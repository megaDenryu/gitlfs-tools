//! PC設定ファイルが存在しない場合と、解析に失敗する場合を区別するテスト。

use gitlfs_tools_config::{PC設定の場所, 設定エラー};

#[test]
fn pc設定ファイルが存在しない場合は未検出として区別される() -> Result<(), Box<dyn std::error::Error>> {
    let 空のディレクトリ = tempfile::tempdir()?;

    let 結果 = PC設定の場所::ディレクトリを指定して生成する(空のディレクトリ.path()).読み込む();

    assert!(matches!(結果, Err(設定エラー::PC設定未検出)));
    Ok(())
}

#[test]
fn pc設定ファイルの構文が不正な場合は解析失敗として区別される() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    std::fs::write(ディレクトリ.path().join("config.toml"), "this is not valid toml =====")?;

    let 結果 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む();

    assert!(matches!(結果, Err(設定エラー::解析失敗 { .. })));
    Ok(())
}
