//! セッション開始後に`init`が再送された場合、応答行を必ず1つ返すことを確かめる
//! （Issue #7是正3。1要求1応答の同期プロトコルであり、無応答のまま次を読みに行くと
//! 応答を待つGit LFS側がハングするため）。実rcloneが必要なため、PATHまたは
//! `LFS_RCLONE_TEST_EXECUTABLE`から実行ファイルを解決する。解決できなければ、
//! `LFS_RCLONE_SKIP_INTEGRATION`が設定されている場合に限り読み飛ばし、それ以外は失敗させる
//! （`common::rclone_executable`参照）。

mod common;

use std::path::Path;

#[test]
fn セッション開始後のinit再送にも応答行を返す() -> Result<(), Box<dyn std::error::Error>> {
    let rclone実行ファイル = match common::rclone_executable::実行ファイルを解決する()? {
        common::rclone_executable::実行ファイル解決::明示された場所(パス) => Some(パス),
        common::rclone_executable::実行ファイル解決::PATH解決 => None,
        common::rclone_executable::実行ファイル解決::読み飛ばす => {
            eprintln!("LFS_RCLONE_SKIP_INTEGRATION が設定されているため、結合テストを読み飛ばします。");
            return Ok(());
        }
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = common::fixtures::ドライブとパスへ分ける(保管先ルート.path())?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("resend-init")?;
    let pc設定 =
        common::fixtures::pc設定ディレクトリを作る("resend-init", &ドライブ, &残り, 一時ディレクトリ.path(), rclone実行ファイル.as_deref())?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;

    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "upload", "remote": "origin"}))?;
    assert_eq!(プロセス.一行受け取る()?, serde_json::json!({}));

    // セッション開始後にinitを再送する。仕様上Git LFSはこれを行わないが、無応答のまま
    // 次の要求読み取りへ進むとGit LFS側がハングするため、応答行が必ず返ることを確かめる。
    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "upload", "remote": "origin"}))?;
    let 再送応答 = プロセス.一行受け取る()?;
    assert_eq!(再送応答, serde_json::json!({}), "init再送にも応答行が返るべき: {再送応答:?}");

    プロセス.一行送る(&serde_json::json!({"event": "terminate"}))?;
    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(終了状態.success(), "init再送後もセッションは継続し、terminateで正常終了するべき");
    Ok(())
}
