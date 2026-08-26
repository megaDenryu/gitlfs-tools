//! `資産転送サービス::ダウンロードする`のユースケーステスト。
//! rcloneを使わず、`common::偽の保管庫`だけで検証する（Issue #6テスト節）。

mod common;

use lfs_rclone_domain::{一時ディレクトリ, 保管エラー};
use lfs_rclone_transfer::{ダウンロード要求, 資産転送サービス};

use common::偽の保管庫;

#[test]
fn ダウンロード後にバイト数とsha256を再検証して完了値へ内容を渡す() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"download verification fixture content".to_vec();
    let 識別子文字列 = common::sha256の16進文字列(&内容);
    let バイト数 = u64::try_from(内容.len())?;

    let 保管庫 = 偽の保管庫::生成する();
    保管庫.存在するオブジェクトとして登録する(&識別子文字列, バイト数);
    保管庫.ダウンロード時の内容を登録する(&識別子文字列, 内容.clone());

    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = ダウンロード要求::生成する(&識別子文字列, バイト数)?;
    let 完了 = サービス.ダウンロードする(要求)?;

    assert_eq!(完了.バイト数().値(), バイト数);
    assert_eq!(std::fs::read(完了.パス())?, 内容);
    Ok(())
}

#[test]
fn 未存在オブジェクトのダウンロードは未存在エラーになる() -> Result<(), Box<dyn std::error::Error>> {
    let 識別子文字列 = common::sha256の16進文字列(b"never uploaded fixture");
    let 保管庫 = 偽の保管庫::生成する();

    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = ダウンロード要求::生成する(&識別子文字列, 12)?;
    let 結果 = サービス.ダウンロードする(要求);

    assert!(matches!(結果, Err(保管エラー::未存在 { .. })));
    assert_eq!(std::fs::read_dir(一時ディレクトリガード.path())?.count(), 0);
    Ok(())
}

#[test]
fn サイズ不一致のダウンロードは未存在とは別の整合性エラーになる() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"size mismatch on download fixture".to_vec();
    let 識別子文字列 = common::sha256の16進文字列(&内容);
    let 保管先の実バイト数 = u64::try_from(内容.len())?;
    let 要求するバイト数 = 保管先の実バイト数 + 1;

    let 保管庫 = 偽の保管庫::生成する();
    保管庫.存在するオブジェクトとして登録する(&識別子文字列, 保管先の実バイト数);

    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = ダウンロード要求::生成する(&識別子文字列, 要求するバイト数)?;
    let 結果 = サービス.ダウンロードする(要求);

    assert!(matches!(結果, Err(保管エラー::整合性(_))));
    assert!(!matches!(結果, Err(保管エラー::未存在 { .. })));
    assert_eq!(std::fs::read_dir(一時ディレクトリガード.path())?.count(), 0);
    Ok(())
}

#[test]
fn 破損したダウンロードを成功として返さず一時ファイルを片づける() -> Result<(), Box<dyn std::error::Error>> {
    let 要求どおりの内容 = b"content the caller actually expects";
    let 保管先が実際に返す内容 = b"content the caller actually gets---".to_vec();
    assert_eq!(要求どおりの内容.len(), 保管先が実際に返す内容.len());

    let 識別子文字列 = common::sha256の16進文字列(要求どおりの内容);
    let バイト数 = u64::try_from(要求どおりの内容.len())?;

    let 保管庫 = 偽の保管庫::生成する();
    保管庫.存在するオブジェクトとして登録する(&識別子文字列, バイト数);
    保管庫.ダウンロード時の内容を登録する(&識別子文字列, 保管先が実際に返す内容);

    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = ダウンロード要求::生成する(&識別子文字列, バイト数)?;
    let 結果 = サービス.ダウンロードする(要求);

    assert!(結果.is_err());
    assert_eq!(std::fs::read_dir(一時ディレクトリガード.path())?.count(), 0);
    Ok(())
}

#[test]
fn ダウンロードのたびに固有の一時ファイルへ書き込み既存パスを再利用しない() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"unique temp path fixture content".to_vec();
    let 識別子文字列 = common::sha256の16進文字列(&内容);
    let バイト数 = u64::try_from(内容.len())?;

    let 保管庫 = 偽の保管庫::生成する();
    保管庫.存在するオブジェクトとして登録する(&識別子文字列, バイト数);
    保管庫.ダウンロード時の内容を登録する(&識別子文字列, 内容);

    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 完了1 = サービス.ダウンロードする(ダウンロード要求::生成する(&識別子文字列, バイト数)?)?;
    let 完了2 = サービス.ダウンロードする(ダウンロード要求::生成する(&識別子文字列, バイト数)?)?;

    assert_ne!(完了1.パス(), 完了2.パス());
    assert_eq!(std::fs::read_dir(一時ディレクトリガード.path())?.count(), 2);
    Ok(())
}
