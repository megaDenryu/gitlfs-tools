//! 使われなくなった`temp_directory`の扱い: 書いてあっても読み込みは成功し（既存のPC設定
//! ファイルを壊さない）、使われなくなった項目として報告されること。省略した設定も読める。

mod common;

use gitlfs_tools_config::{使われなくなった設定項目, PC設定の場所};
use gitlfs_tools_domain::プロファイル名;

use common::pc設定ディレクトリを作る;

#[test]
fn temp_directoryが残っていても読み込めて使われなくなった項目として報告される() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                temp_directory = \"D:/large-assets-tmp\"\n\
                transfer_timeout_seconds = 1\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;

    let pc設定 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む()?;
    let プロファイル = pc設定.プロファイルを解決する(&プロファイル名::生成する("x")?)?;

    assert_eq!(プロファイル.使われなくなった項目一覧(), &[使われなくなった設定項目::一時ディレクトリの指定]);
    assert_eq!(使われなくなった設定項目::一時ディレクトリの指定.キー名(), "temp_directory");
    Ok(())
}

#[test]
fn temp_directoryを省略したpc設定も読み込める() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                transfer_timeout_seconds = 1\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;

    let pc設定 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む()?;
    let プロファイル = pc設定.プロファイルを解決する(&プロファイル名::生成する("x")?)?;

    assert!(プロファイル.使われなくなった項目一覧().is_empty());
    Ok(())
}
