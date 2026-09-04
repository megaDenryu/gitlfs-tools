//! ローカルディレクトリ方式（PC設定`storage = "local"`）でのプロトコル往復。init・upload・
//! download・terminateを1つのプロセスで通し、rcloneを一度も起動せずに転送できることと、
//! 保管先の一時領域に残骸が残らないことを確かめる。rcloneに依存しないため、この結合テストは
//! 環境によらず既定で実行する（読み飛ばしの経路を持たない）。

mod common;

use std::path::Path;

use serde_json::json;

#[test]
fn ローカルディレクトリ方式でuploadとdownloadが往復する() -> Result<(), Box<dyn std::error::Error>> {
    let 保管先ルート = tempfile::tempdir()?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("local-roundtrip")?;
    let pc設定 =
        common::fixtures::ローカル方式のpc設定ディレクトリを作る("local-roundtrip", 保管先ルート.path(), 一時ディレクトリ.path())?;
    let 内容: &[u8] = b"local storage roundtrip payload";
    let (アップロード元, oid, size) = common::payload::アップロード元を作る(作業ツリー.path(), "payload.bin", 内容)?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&json!({"event": "init", "operation": "upload", "remote": "origin"}))?;
    assert_eq!(プロセス.一行受け取る()?, json!({}));

    アップロードして成功を確かめる(&mut プロセス, &oid, size, &アップロード元)?; // 新規
    アップロードして成功を確かめる(&mut プロセス, &oid, size, &アップロード元)?; // 既存でも成功する

    プロセス.一行送る(&json!({"event": "download", "oid": oid, "size": size, "action": null}))?;
    let 進捗 = プロセス.一行受け取る()?;
    assert_eq!(進捗["bytesSoFar"], size);
    let 完了 = プロセス.一行受け取る()?;
    assert_eq!(完了["event"], "complete");
    assert_eq!(完了["oid"], oid);
    let 保存先 = 完了["path"].as_str().ok_or("pathが文字列ではありません")?;
    assert_eq!(std::fs::read(保存先)?, 内容);

    プロセス.一行送る(&json!({"event": "terminate"}))?;
    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(終了状態.success());

    let 実体のパス = 保管先ルート.path().join("lfs").join("objects").join("sha256").join(&oid[0..2]).join(&oid[2..4]).join(&oid);
    assert_eq!(std::fs::read(実体のパス)?, 内容, "最終オブジェクトの位置はrclone方式と同じ綴りであるべき");

    let 一時領域 = 保管先ルート.path().join("lfs").join("tmp");
    assert_eq!(std::fs::read_dir(&一時領域)?.count(), 0, "保管先の一時領域に残骸を残してはならない");
    Ok(())
}

#[test]
fn 保管先ルートが存在しなければinitが失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 親 = tempfile::tempdir()?;
    let 存在しない保管先ルート = 親.path().join("mounted-drive-is-absent");
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("local-absent-root")?;
    let pc設定 = common::fixtures::ローカル方式のpc設定ディレクトリを作る(
        "local-absent-root",
        &存在しない保管先ルート,
        一時ディレクトリ.path(),
    )?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&json!({"event": "init", "operation": "upload", "remote": "origin"}))?;
    let 応答 = プロセス.一行受け取る()?;

    assert!(応答.get("error").is_some(), "保管先ルートが無いのにinitが成功してはならない: {応答:?}");
    assert!(!存在しない保管先ルート.exists(), "基点が存在しないときに勝手に作ってはならない");

    プロセス.一行送る(&json!({"event": "terminate"}))?;
    プロセス.終了を待って後始末する()?;
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
