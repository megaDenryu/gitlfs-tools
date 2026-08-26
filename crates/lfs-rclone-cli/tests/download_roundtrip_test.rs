//! download: 成功、未存在、整合性失敗、失敗後の継続、進捗の最終値、terminate後に失敗分の
//! 一時ファイルが残らないことを確かめる。実rcloneが必要なため、`LFS_RCLONE_TEST_EXECUTABLE`
//! が無ければ読み飛ばす。

mod common;

use std::path::Path;

use serde_json::json;

#[test]
fn downloadの成功未存在整合性失敗と失敗後の継続を確かめる() -> Result<(), Box<dyn std::error::Error>> {
    let Some(rclone実行ファイル) = common::fixtures::rclone実行ファイルのパスを探す() else {
        eprintln!("LFS_RCLONE_TEST_EXECUTABLE が未設定のため、結合テストを読み飛ばします。");
        return Ok(());
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = common::fixtures::ドライブとパスへ分ける(保管先ルート.path())?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("download-roundtrip")?;
    let pc設定 = common::fixtures::pc設定ディレクトリを作る(
        "download-roundtrip",
        &ドライブ,
        &残り,
        一時ディレクトリ.path(),
        Some(&rclone実行ファイル),
    )?;
    let 内容: &[u8] = b"download roundtrip test payload";
    let (アップロード元, oid, size) = common::payload::アップロード元を作る(作業ツリー.path(), "payload.bin", 内容)?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&json!({"event": "init", "operation": "download", "remote": "origin"}))?;
    assert_eq!(プロセス.一行受け取る()?, json!({}));

    // 事前にアップロードしてストレージへ実体を置く。
    プロセス.一行送る(&json!({"event": "upload", "oid": oid, "size": size, "path": アップロード元, "action": null}))?;
    プロセス.一行受け取る()?;
    プロセス.一行受け取る()?;

    ダウンロードして成功を確かめる(&mut プロセス, &oid, size, 内容)?;

    let 未存在oid = "f".repeat(64);
    プロセス.一行送る(&json!({"event": "download", "oid": 未存在oid, "size": 1, "action": null}))?;
    let 未存在応答 = プロセス.一行受け取る()?;
    assert_eq!(未存在応答["event"], "complete");
    assert!(未存在応答.get("error").is_some());

    プロセス.一行送る(&json!({"event": "download", "oid": oid, "size": size + 1, "action": null}))?;
    let 整合性失敗応答 = プロセス.一行受け取る()?;
    assert!(整合性失敗応答.get("error").is_some(), "サイズ偽装は整合性エラーになるべき: {整合性失敗応答:?}");

    ダウンロードして成功を確かめる(&mut プロセス, &oid, size, 内容)?; // 失敗後も継続できる

    プロセス.一行送る(&json!({"event": "terminate"}))?;
    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(終了状態.success());

    let 残存ファイル数 = std::fs::read_dir(一時ディレクトリ.path())?.count();
    assert_eq!(残存ファイル数, 2, "成功した2件のdownload先だけが残るべき(失敗分は削除済み)");
    Ok(())
}

fn ダウンロードして成功を確かめる(
    プロセス: &mut common::process::プロトコルテストプロセス,
    oid: &str,
    size: u64,
    期待する内容: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    プロセス.一行送る(&json!({"event": "download", "oid": oid, "size": size, "action": null}))?;
    let 進捗 = プロセス.一行受け取る()?;
    assert_eq!(進捗["bytesSoFar"], size);
    let 完了 = プロセス.一行受け取る()?;
    assert_eq!(完了["event"], "complete");
    assert_eq!(完了["oid"], oid);
    let 保存先 = 完了["path"].as_str().ok_or("pathが文字列ではありません")?;
    assert_eq!(std::fs::read(保存先)?, 期待する内容);
    Ok(())
}
