//! PC設定`config.toml`の読み込み時の妥当性検査(schema版・未知キー)のテスト。

mod common;

use gitlfs_tools_config::{PC設定の場所, 設定エラー};

use common::pc設定ディレクトリを作る;

#[test]
fn 未対応のschema版を拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 2\n\
                [profiles.x]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                transfer_timeout_seconds = 1\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;

    let 結果 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む();

    assert!(matches!(
        結果,
        Err(設定エラー::未対応スキーマ版 { 受信した版: 2, 受理できる版: 1 })
    ));
    Ok(())
}

#[test]
fn 未知キーを含むpc設定を拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                transfer_timeout_seconds = 1\n\
                oauth_token = \"leaked-token\"\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;

    let 結果 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む();

    assert!(matches!(結果, Err(設定エラー::解析失敗 { .. })));
    Ok(())
}
