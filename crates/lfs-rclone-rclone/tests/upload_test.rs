//! `アップロードする`が一時パスを経由して最終パスへ置くこと、既存時に転送しないことを
//! 偽rclone実行ファイルを通して確かめる。

mod common;

use std::io::Write;
use std::time::Duration;

use lfs_rclone_domain::{オブジェクト識別子, 期待バイト数, 検証前のローカルファイル, 検証済みローカルファイル};
use lfs_rclone_storage_port::{アップロード結果, オブジェクト保管庫};
use sha2::{Digest, Sha256};

fn 内容を書いた検証済みローカルファイルを作る(
    内容: &[u8],
) -> Result<(tempfile::NamedTempFile, オブジェクト識別子, 検証済みローカルファイル), Box<dyn std::error::Error>> {
    let mut ファイル = tempfile::NamedTempFile::new()?;
    ファイル.write_all(内容)?;

    let ダイジェスト = Sha256::digest(内容);
    let 十六進文字列: String = ダイジェスト.iter().map(|バイト| format!("{バイト:02x}")).collect();
    let 識別子 = オブジェクト識別子::生成する(&十六進文字列)?;
    let バイト数 = 期待バイト数::生成する(u64::try_from(内容.len())?);

    let 検証前 = 検証前のローカルファイル::生成する(ファイル.path());
    let 検証済み = 検証前.検証する(&識別子, バイト数)?;

    Ok((ファイル, 識別子, 検証済み))
}

#[test]
fn 未存在なら一時パスを経由して最終パスへ置く() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("upload-fresh")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;
    let (_ファイル, 識別子, 検証済み) = 内容を書いた検証済みローカルファイルを作る(b"issue5 upload fixture")?;
    指示.最終化後のバイト数を仕込む(検証済み.バイト数().値())?;

    let 結果 = 保管庫.アップロードする(&識別子, &検証済み)?;

    assert_eq!(結果, アップロード結果::転送済み);

    let 呼び出し一覧 = 指示.記録済み呼び出し一覧を読む();
    let サブコマンド一覧: Vec<&str> = 呼び出し一覧.iter().map(|引数| 引数[0].as_str()).collect();
    assert_eq!(サブコマンド一覧, vec!["lsjson", "copyto", "moveto", "lsjson"]);

    let copyto呼び出し = &呼び出し一覧[1];
    assert!(copyto呼び出し.contains(&"--ignore-times".to_owned()));
    assert!(copyto呼び出し.last().is_some_and(|最後| 最後.contains("/lfs/tmp/")));

    let moveto呼び出し = &呼び出し一覧[2];
    assert!(moveto呼び出し[moveto呼び出し.len() - 2].contains("/lfs/tmp/"));
    assert!(moveto呼び出し.last().is_some_and(|最後| 最後.contains("/lfs/objects/sha256/")));

    Ok(())
}

#[test]
fn 既に同じサイズで存在すれば転送せず既存を返す() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("upload-already")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    let (_ファイル, 識別子, 検証済み) = 内容を書いた検証済みローカルファイルを作る(b"issue5 already present fixture")?;
    指示.既存として仕込む(検証済み.バイト数().値())?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 結果 = 保管庫.アップロードする(&識別子, &検証済み)?;

    assert_eq!(結果, アップロード結果::既存);

    let 呼び出し一覧 = 指示.記録済み呼び出し一覧を読む();
    let サブコマンド一覧: Vec<&str> = 呼び出し一覧.iter().map(|引数| 引数[0].as_str()).collect();
    assert_eq!(サブコマンド一覧, vec!["lsjson"]);

    Ok(())
}

#[test]
fn 既に違うサイズで存在すれば整合性エラーで転送しない() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("upload-corrupt")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    let (_ファイル, 識別子, 検証済み) = 内容を書いた検証済みローカルファイルを作る(b"issue5 corrupt existing fixture")?;
    指示.既存として仕込む(検証済み.バイト数().値() + 1)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 結果 = 保管庫.アップロードする(&識別子, &検証済み);

    assert!(matches!(
        結果,
        Err(lfs_rclone_domain::保管エラー::整合性(lfs_rclone_domain::整合性エラー::バイト数の不一致 { .. }))
    ));

    let 呼び出し一覧 = 指示.記録済み呼び出し一覧を読む();
    let サブコマンド一覧: Vec<&str> = 呼び出し一覧.iter().map(|引数| 引数[0].as_str()).collect();
    assert_eq!(サブコマンド一覧, vec!["lsjson"]);

    Ok(())
}
