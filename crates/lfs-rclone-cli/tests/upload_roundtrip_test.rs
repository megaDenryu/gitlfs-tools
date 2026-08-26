//! upload: 成功(新規)、AlreadyPresent、整合性失敗、失敗後の継続、進捗の最終値を確かめる。
//! 実rcloneが必要なため、`LFS_RCLONE_TEST_EXECUTABLE`が無ければ読み飛ばす
//! （`lfs-rclone-rclone`の結合テストと同じ方針）。

mod common;

use std::path::Path;

use serde_json::json;

#[test]
fn uploadの成功既存整合性失敗と失敗後の継続を確かめる() -> Result<(), Box<dyn std::error::Error>> {
    let Some(rclone実行ファイル) = common::fixtures::rclone実行ファイルのパスを探す() else {
        eprintln!("LFS_RCLONE_TEST_EXECUTABLE が未設定のため、結合テストを読み飛ばします。");
        return Ok(());
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = common::fixtures::ドライブとパスへ分ける(保管先ルート.path())?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("upload-roundtrip")?;
    let pc設定 =
        common::fixtures::pc設定ディレクトリを作る("upload-roundtrip", &ドライブ, &残り, 一時ディレクトリ.path(), Some(&rclone実行ファイル))?;
    let (アップロード元, oid, size) =
        common::payload::アップロード元を作る(作業ツリー.path(), "payload.bin", b"upload roundtrip test payload")?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&json!({"event": "init", "operation": "upload", "remote": "origin"}))?;
    assert_eq!(プロセス.一行受け取る()?, json!({}));

    アップロードして成功を確かめる(&mut プロセス, &oid, size, &アップロード元)?; // 新規
    アップロードして成功を確かめる(&mut プロセス, &oid, size, &アップロード元)?; // AlreadyPresent

    プロセス.一行送る(&json!({"event": "upload", "oid": oid, "size": size + 1, "path": アップロード元, "action": null}))?;
    let 失敗応答 = プロセス.一行受け取る()?;
    assert_eq!(失敗応答["event"], "complete");
    assert_eq!(失敗応答["oid"], oid);
    assert!(失敗応答.get("error").is_some(), "サイズ偽装は整合性エラーになるべき: {失敗応答:?}");

    アップロードして成功を確かめる(&mut プロセス, &oid, size, &アップロード元)?; // 失敗後も継続できる

    プロセス.一行送る(&json!({"event": "terminate"}))?;
    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(終了状態.success());
    Ok(())
}

fn アップロードして成功を確かめる(
    プロセス: &mut common::process::プロトコルテストプロセス,
    oid: &str,
    size: u64,
    パス: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    プロセス.一行送る(&json!({"event": "upload", "oid": oid, "size": size, "path": パス, "action": null}))?;
    let 進捗 = プロセス.一行受け取る()?;
    assert_eq!(進捗["event"], "progress");
    assert_eq!(進捗["bytesSoFar"], size);
    let 完了 = プロセス.一行受け取る()?;
    assert_eq!(完了, json!({"event": "complete", "oid": oid}));
    Ok(())
}
