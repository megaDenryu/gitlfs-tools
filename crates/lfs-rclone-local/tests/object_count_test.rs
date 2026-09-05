//! `保管しているオブジェクトの総数を数える`が、オブジェクト置き場の配下だけを再帰的に数える
//! ことを確かめる。一時アップロード領域を数に入れないこと、置き場が未作成なら0件として扱う
//! こと、基点が無ければ設定不備として失敗することも見る。

mod common;

use lfs_rclone_domain::保管エラー;
use lfs_rclone_storage_port::オブジェクト保管庫;

#[test]
fn 置き場が未作成なら0件を返す() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;

    let 総数 = 保管庫.保管しているオブジェクトの総数を数える()?;

    assert_eq!(総数.件数(), 0);
    Ok(())
}

#[test]
fn 置き場の配下を再帰的に数え一時領域は数えない() -> Result<(), Box<dyn std::error::Error>> {
    let ルート = tempfile::tempdir()?;
    let 保管庫 = common::保管庫を作る(ルート.path())?;

    for 内容 in [b"count one".as_slice(), b"count two".as_slice(), b"count three".as_slice()] {
        let 識別子 = common::内容から識別子を求める(内容)?;
        let 配置先 = common::最終オブジェクトのパス(ルート.path(), &識別子);
        std::fs::create_dir_all(配置先.parent().ok_or("親ディレクトリがありません")?)?;
        std::fs::write(&配置先, 内容)?;
    }
    let 一時領域 = common::一時アップロード領域のパス(ルート.path());
    std::fs::create_dir_all(&一時領域)?;
    std::fs::write(一時領域.join("転送途中の残骸"), b"leftover")?;

    let 総数 = 保管庫.保管しているオブジェクトの総数を数える()?;

    assert_eq!(総数.件数(), 3, "オブジェクト置き場の配下だけを数えるべき");
    Ok(())
}

#[test]
fn 基点が存在しなければ設定不備として失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 親 = tempfile::tempdir()?;
    let 存在しないルート = 親.path().join("mounted-drive-is-absent");
    let 保管庫 = common::保管庫を作る(&存在しないルート)?;

    let 結果 = 保管庫.保管しているオブジェクトの総数を数える();

    assert!(matches!(結果, Err(保管エラー::設定不備 { .. })), "実際の結果: {結果:?}");
    Ok(())
}
