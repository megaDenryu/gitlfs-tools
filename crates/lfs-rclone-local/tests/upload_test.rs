//! `アップロードする`が中間ディレクトリを作りながら一時パスを経由して最終パスへ置くこと、
//! 既に同じ内容があれば転送しないことを確かめる。

mod common;

use std::time::SystemTime;

use lfs_rclone_storage_port::{アップロード結果, オブジェクト保管庫};

#[test]
fn 中間ディレクトリが無くても最終パスへ置き一時領域を残さない() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let 内容: &[u8] = b"local upload fixture";
    let (_ファイル, 識別子, 検証済み) = common::内容を書いた検証済みローカルファイルを作る(内容)?;

    let 結果 = 保管庫.アップロードする(&識別子, &検証済み)?;

    assert_eq!(結果, アップロード結果::転送済み);
    let 配置先 = common::最終オブジェクトのパス(ルート.path(), &識別子);
    assert_eq!(std::fs::read(&配置先)?, 内容);

    let 一時領域 = common::一時アップロード領域のパス(ルート.path());
    let 一時領域の残存件数 = if 一時領域.is_dir() { std::fs::read_dir(&一時領域)?.count() } else { 0 };
    assert_eq!(一時領域の残存件数, 0, "最終パスへ移した後の一時領域は空であるべき");
    Ok(())
}

#[test]
fn 既に同じ内容があれば転送せず既存を返す() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;
    let (_ファイル, 識別子, 検証済み) = common::内容を書いた検証済みローカルファイルを作る(b"local upload twice")?;
    assert_eq!(保管庫.アップロードする(&識別子, &検証済み)?, アップロード結果::転送済み);

    let 配置先 = common::最終オブジェクトのパス(ルート.path(), &識別子);
    let 一回目の更新時刻 = std::fs::metadata(&配置先)?.modified()?;

    let 二回目の結果 = 保管庫.アップロードする(&識別子, &検証済み)?;

    assert_eq!(二回目の結果, アップロード結果::既存);
    let 二回目の更新時刻: SystemTime = std::fs::metadata(&配置先)?.modified()?;
    assert_eq!(一回目の更新時刻, 二回目の更新時刻, "既存なら書き直してはならない");
    Ok(())
}

#[test]
fn 保管先の基点が存在しなければアップロードは失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 親 = tempfile::tempdir()?;
    let 存在しないルート = 親.path().join("mounted-drive-is-absent");
    let 保管庫 = common::保管庫を作る(&存在しないルート)?;
    let (_ファイル, 識別子, 検証済み) = common::内容を書いた検証済みローカルファイルを作る(b"absent root upload")?;

    let 結果 = 保管庫.アップロードする(&識別子, &検証済み);

    assert!(結果.is_err(), "基点が無いのに成功してはならない");
    assert!(!存在しないルート.exists(), "基点が存在しないときに勝手に作ってはならない");
    Ok(())
}
