//! `資産転送サービス::アップロードする`のユースケーステスト。
//! rcloneを使わず、`common::偽の保管庫`だけで検証する（Issue #6テスト節）。

mod common;

use lfs_rclone_domain::{一時ディレクトリ, 保管エラー, 整合性エラー, 検証前のローカルファイル};
use lfs_rclone_storage_port::アップロード結果;
use lfs_rclone_transfer::{アップロード要求, 資産転送サービス};

use common::偽の保管庫;

#[test]
fn 同一識別子のアップロードは冪等であり保管庫への転送は一度だけ起きる() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"upload idempotency fixture content";
    let 識別子文字列 = common::sha256の16進文字列(内容);
    let バイト数 = u64::try_from(内容.len())?;

    let 入力ディレクトリ = tempfile::tempdir()?;
    let 入力パス = common::内容を書いたファイルを作る(入力ディレクトリ.path(), "object.bin", 内容)?;

    let 保管庫 = 偽の保管庫::生成する();
    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求を作る = |入力パス: &std::path::Path| {
        アップロード要求::生成する(&識別子文字列, バイト数, 検証前のローカルファイル::生成する(入力パス))
    };

    let 一回目 = サービス.アップロードする(要求を作る(&入力パス)?)?;
    assert_eq!(一回目.結果(), アップロード結果::転送済み);

    let 二回目 = サービス.アップロードする(要求を作る(&入力パス)?)?;
    assert_eq!(二回目.結果(), アップロード結果::既存);

    assert_eq!(保管庫.アップロード呼び出し回数(&識別子文字列), 1);
    Ok(())
}

#[test]
fn バイト数が違う入力ファイルはアップロード前に拒否され保管庫を呼ばない() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"size mismatch fixture content";
    let 識別子文字列 = common::sha256の16進文字列(内容);
    let 誤ったバイト数 = u64::try_from(内容.len())? + 1;

    let 入力ディレクトリ = tempfile::tempdir()?;
    let 入力パス = common::内容を書いたファイルを作る(入力ディレクトリ.path(), "object.bin", 内容)?;

    let 保管庫 = 偽の保管庫::生成する();
    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = アップロード要求::生成する(&識別子文字列, 誤ったバイト数, 検証前のローカルファイル::生成する(&入力パス))?;
    let 結果 = サービス.アップロードする(要求);

    assert!(matches!(
        結果,
        Err(保管エラー::整合性(整合性エラー::バイト数の不一致 { .. }))
    ));
    assert_eq!(保管庫.アップロード呼び出し回数(&識別子文字列), 0);
    Ok(())
}

#[test]
fn 内容が異なる入力ファイルはハッシュ不一致でアップロード前に拒否される() -> Result<(), Box<dyn std::error::Error>> {
    let ディスク上の実際の内容 = b"content-actually-on-disk---";
    let 要求と食い違う内容 = b"content-declared-instead---";
    assert_eq!(ディスク上の実際の内容.len(), 要求と食い違う内容.len());

    let 食い違う識別子文字列 = common::sha256の16進文字列(要求と食い違う内容);
    let バイト数 = u64::try_from(ディスク上の実際の内容.len())?;

    let 入力ディレクトリ = tempfile::tempdir()?;
    let 入力パス = common::内容を書いたファイルを作る(入力ディレクトリ.path(), "object.bin", ディスク上の実際の内容)?;

    let 保管庫 = 偽の保管庫::生成する();
    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = アップロード要求::生成する(&食い違う識別子文字列, バイト数, 検証前のローカルファイル::生成する(&入力パス))?;
    let 結果 = サービス.アップロードする(要求);

    assert!(matches!(
        結果,
        Err(保管エラー::整合性(整合性エラー::内容ハッシュの不一致 { .. }))
    ));
    assert_eq!(保管庫.アップロード呼び出し回数(&食い違う識別子文字列), 0);
    Ok(())
}

#[test]
fn 保管先の実サイズが要求と食い違う場合はアップロード前に拒否される() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"already present with different size";
    let 識別子文字列 = common::sha256の16進文字列(内容);
    let バイト数 = u64::try_from(内容.len())?;

    let 入力ディレクトリ = tempfile::tempdir()?;
    let 入力パス = common::内容を書いたファイルを作る(入力ディレクトリ.path(), "object.bin", 内容)?;

    let 保管庫 = 偽の保管庫::生成する();
    保管庫.存在するオブジェクトとして登録する(&識別子文字列, バイト数 + 1);

    let 一時ディレクトリガード = tempfile::tempdir()?;
    let サービス = 資産転送サービス::生成する(
        &保管庫,
        一時ディレクトリ::生成する(一時ディレクトリガード.path()),
    );

    let 要求 = アップロード要求::生成する(&識別子文字列, バイト数, 検証前のローカルファイル::生成する(&入力パス))?;
    let 結果 = サービス.アップロードする(要求);

    assert!(matches!(
        結果,
        Err(保管エラー::整合性(整合性エラー::バイト数の不一致 { .. }))
    ));
    assert_eq!(保管庫.アップロード呼び出し回数(&識別子文字列), 0);
    Ok(())
}
