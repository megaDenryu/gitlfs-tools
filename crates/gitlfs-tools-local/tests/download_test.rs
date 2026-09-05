//! `ダウンロードする`が指定した一時ファイルパスへ取得すること、未存在を区別すること、
//! 既存のファイルを無言で上書きしないことを確かめる。

mod common;

use gitlfs_tools_domain::{一時ディレクトリ, 保管エラー, 期待バイト数};
use gitlfs_tools_storage_port::オブジェクト保管庫;

#[test]
fn 存在すれば指定した一時ファイルパスへ取得する() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let 内容: &[u8] = b"local download fixture";
    let (_ファイル, 識別子, 検証済み) = common::内容を書いた検証済みローカルファイルを作る(内容)?;
    保管庫.アップロードする(&識別子, &検証済み)?;

    let 作業ディレクトリ = tempfile::tempdir()?;
    let 保存先 = 一時ディレクトリ::生成する(作業ディレクトリ.path()).固有の一時ファイルパスを払い出す();

    保管庫.ダウンロードする(&識別子, 検証済み.バイト数(), &保存先)?;

    assert_eq!(std::fs::read(保存先.パス())?, 内容);
    Ok(())
}

#[test]
fn 未存在なら未存在エラーになる() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let 識別子 = common::内容から識別子を求める(b"local download missing")?;
    let 作業ディレクトリ = tempfile::tempdir()?;
    let 保存先 = 一時ディレクトリ::生成する(作業ディレクトリ.path()).固有の一時ファイルパスを払い出す();

    let 結果 = 保管庫.ダウンロードする(&識別子, 期待バイト数::生成する(10), &保存先);

    assert!(matches!(結果, Err(保管エラー::未存在 { .. })), "実際の結果: {結果:?}");
    Ok(())
}

#[test]
fn 既存のダウンロード先を無言で上書きしない() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let (_ファイル, 識別子, 検証済み) = common::内容を書いた検証済みローカルファイルを作る(b"local download conflict")?;
    保管庫.アップロードする(&識別子, &検証済み)?;

    let 作業ディレクトリ = tempfile::tempdir()?;
    let 保存先 = 一時ディレクトリ::生成する(作業ディレクトリ.path()).固有の一時ファイルパスを払い出す();
    let 先客の内容: &[u8] = b"already there";
    std::fs::write(保存先.パス(), 先客の内容)?;

    let 結果 = 保管庫.ダウンロードする(&識別子, 検証済み.バイト数(), &保存先);

    assert!(matches!(結果, Err(保管エラー::ローカル入出力 { .. })), "実際の結果: {結果:?}");
    assert_eq!(std::fs::read(保存先.パス())?, 先客の内容);
    Ok(())
}
