//! `存在を確認する`が未存在・存在・サイズの不一致を区別すること、および保管先の基点が
//! 実在しないときに明示的に失敗することを確かめる。

mod common;

use gitlfs_tools_domain::{オブジェクト状態, 保管エラー, 期待バイト数};
use gitlfs_tools_storage_port::オブジェクト保管庫;

#[test]
fn 未存在なら未存在を返す() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let 識別子 = common::内容から識別子を求める(b"existence check missing")?;

    let 状態 = 保管庫.存在を確認する(&識別子, 期待バイト数::生成する(100))?;

    assert_eq!(状態, オブジェクト状態::未存在);
    Ok(())
}

#[test]
fn 同じバイト数で存在すれば存在を返す() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let 内容: &[u8] = b"existence check present";
    let 識別子 = common::内容から識別子を求める(内容)?;
    let 配置先 = common::最終オブジェクトのパス(ルート.path(), &識別子);
    std::fs::create_dir_all(配置先.parent().ok_or("親ディレクトリがありません")?)?;
    std::fs::write(&配置先, 内容)?;

    let 状態 = 保管庫.存在を確認する(&識別子, 期待バイト数::生成する(u64::try_from(内容.len())?))?;

    assert_eq!(状態, オブジェクト状態::存在);
    Ok(())
}

#[test]
fn バイト数が違えばサイズの不一致を返す() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let 内容: &[u8] = b"existence check mismatched";
    let 識別子 = common::内容から識別子を求める(内容)?;
    let 配置先 = common::最終オブジェクトのパス(ルート.path(), &識別子);
    std::fs::create_dir_all(配置先.parent().ok_or("親ディレクトリがありません")?)?;
    std::fs::write(&配置先, 内容)?;

    let 状態 = 保管庫.存在を確認する(&識別子, 期待バイト数::生成する(1))?;

    assert_eq!(状態, オブジェクト状態::サイズの不一致 { 実サイズ: 期待バイト数::生成する(u64::try_from(内容.len())?) });
    Ok(())
}

#[test]
fn 保管先の基点が存在しなければ設定不備として失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 親 = tempfile::tempdir()?;
    let 存在しないルート = 親.path().join("mounted-drive-is-absent");
    let 保管庫 = common::保管庫を作る(&存在しないルート)?;
    let 識別子 = common::内容から識別子を求める(b"absent root")?;

    let 結果 = 保管庫.存在を確認する(&識別子, 期待バイト数::生成する(1));

    assert!(matches!(結果, Err(保管エラー::設定不備 { .. })), "実際の結果: {結果:?}");
    assert!(!存在しないルート.exists(), "基点が存在しないときに勝手に作ってはならない");
    Ok(())
}
