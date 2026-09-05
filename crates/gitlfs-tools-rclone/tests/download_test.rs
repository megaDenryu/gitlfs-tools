//! `ダウンロードする`が指定した一時ファイルパスへ転送すること、既存ファイルを無言で
//! 上書きしないことを偽rclone実行ファイルを通して確かめる。

mod common;

use std::time::Duration;

use gitlfs_tools_domain::{一時ディレクトリ, オブジェクト識別子, 保管エラー, 期待バイト数};
use gitlfs_tools_storage_port::オブジェクト保管庫;

fn ダミー識別子() -> Result<オブジェクト識別子, Box<dyn std::error::Error>> {
    Ok(オブジェクト識別子::生成する(&"b".repeat(64))?)
}

#[test]
fn 未存在なら未存在エラーになる() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("download-missing")?;
    let _指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;
    let 作業ディレクトリ = tempfile::tempdir()?;
    let 保存先 = 一時ディレクトリ::生成する(作業ディレクトリ.path()).固有の一時ファイルパスを払い出す();

    let 結果 = 保管庫.ダウンロードする(&ダミー識別子()?, 期待バイト数::生成する(10), &保存先);

    assert!(matches!(結果, Err(保管エラー::未存在 { .. })));
    Ok(())
}

#[test]
fn 存在すれば指定した一時ファイルパスへ転送する() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("download-present")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.既存として仕込む(10)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;
    let 作業ディレクトリ = tempfile::tempdir()?;
    let 保存先 = 一時ディレクトリ::生成する(作業ディレクトリ.path()).固有の一時ファイルパスを払い出す();

    保管庫.ダウンロードする(&ダミー識別子()?, 期待バイト数::生成する(10), &保存先)?;

    let 呼び出し一覧 = 指示.記録済み呼び出し一覧を読む();
    let サブコマンド一覧: Vec<&str> = 呼び出し一覧.iter().map(|引数| 引数[0].as_str()).collect();
    assert_eq!(サブコマンド一覧, vec!["lsjson", "copyto"]);

    let copyto呼び出し = &呼び出し一覧[1];
    let 保存先文字列 = 保存先.パス().to_str().map(str::to_owned).unwrap_or_default();
    assert_eq!(copyto呼び出し.last(), Some(&保存先文字列));

    Ok(())
}

#[test]
fn 保存先が既に存在すれば転送せず失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("download-collide")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.既存として仕込む(10)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;
    let 作業ディレクトリ = tempfile::tempdir()?;
    let 保存先 = 一時ディレクトリ::生成する(作業ディレクトリ.path()).固有の一時ファイルパスを払い出す();
    std::fs::write(保存先.パス(), b"already here")?;

    let 結果 = 保管庫.ダウンロードする(&ダミー識別子()?, 期待バイト数::生成する(10), &保存先);

    assert!(matches!(結果, Err(保管エラー::ローカル入出力 { .. })));

    let 呼び出し一覧 = 指示.記録済み呼び出し一覧を読む();
    let サブコマンド一覧: Vec<&str> = 呼び出し一覧.iter().map(|引数| 引数[0].as_str()).collect();
    assert_eq!(サブコマンド一覧, vec!["lsjson"]);

    Ok(())
}
