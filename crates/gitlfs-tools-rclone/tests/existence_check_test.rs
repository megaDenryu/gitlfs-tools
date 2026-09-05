//! `存在を確認する`が未存在・存在・サイズの不一致を区別することを、偽rclone実行ファイル
//! （`lsjson`相当）を通して確かめる。

mod common;

use std::time::Duration;

use gitlfs_tools_domain::{オブジェクト状態, オブジェクト識別子, 期待バイト数};
use gitlfs_tools_storage_port::オブジェクト保管庫;

fn ダミー識別子() -> Result<オブジェクト識別子, Box<dyn std::error::Error>> {
    Ok(オブジェクト識別子::生成する(&"a".repeat(64))?)
}

#[test]
fn 未存在なら未存在を返す() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("exists-missing")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 状態 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(100))?;

    assert_eq!(状態, オブジェクト状態::未存在);

    let 呼び出し一覧 = 指示.記録済み呼び出し一覧を読む();
    assert_eq!(呼び出し一覧.len(), 1);
    let 記録済み引数: Vec<&str> = 呼び出し一覧[0].iter().map(String::as_str).collect();
    let 期待対象 = 対象文字列(&基底パス, &ダミー識別子()?);
    assert_eq!(記録済み引数, vec!["lsjson", "--files-only", "-q", "--stats", "0", 期待対象.as_str()]);
    Ok(())
}

#[test]
fn 一致するサイズで存在すれば存在を返す() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("exists-present")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.既存として仕込む(100)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 状態 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(100))?;

    assert_eq!(状態, オブジェクト状態::存在);
    Ok(())
}

#[test]
fn サイズが違えばサイズの不一致を返す() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("exists-mismatch")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.既存として仕込む(999)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 状態 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(100))?;

    assert_eq!(
        状態,
        オブジェクト状態::サイズの不一致 { 実サイズ: 期待バイト数::生成する(999) }
    );
    Ok(())
}

/// テストが期待する`lsjson`の対象文字列(`<リモート名>:<保管先オブジェクトパス>`)を
/// `common::偽rclone保管庫を作る`と同じ規則で組み立て直す。
fn 対象文字列(基底パス: &str, 識別子: &オブジェクト識別子) -> String {
    let 十六進 = 識別子.文字列表現();
    format!("fakeremote:{基底パス}/lfs/objects/sha256/{}/{}/{十六進}", &十六進[0..2], &十六進[2..4])
}
