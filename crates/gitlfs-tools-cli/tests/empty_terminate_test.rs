//! init成功後、0件の転送要求のままterminateしたときの往復を確かめる。
//! 実rcloneが必要なため、PATHまたは`GITLFS_TOOLS_TEST_EXECUTABLE`から実行ファイルを解決する。
//! 解決できなければ、`GITLFS_TOOLS_SKIP_INTEGRATION`が設定されている場合に限り読み飛ばし、
//! それ以外は失敗させる（`common::rclone_executable`参照）。

mod common;

use std::path::Path;

#[test]
fn init成功後に0件転送でterminateすると正常終了する() -> Result<(), Box<dyn std::error::Error>> {
    let rclone実行ファイル = match common::rclone_executable::実行ファイルを解決する()? {
        common::rclone_executable::実行ファイル解決::明示された場所(パス) => Some(パス),
        common::rclone_executable::実行ファイル解決::PATH解決 => None,
        common::rclone_executable::実行ファイル解決::読み飛ばす => {
            eprintln!("GITLFS_TOOLS_SKIP_INTEGRATION が設定されているため、結合テストを読み飛ばします。");
            return Ok(());
        }
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = common::fixtures::ドライブとパスへ分ける(保管先ルート.path())?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("empty-terminate")?;
    let pc設定 =
        common::fixtures::pc設定ディレクトリを作る("empty-terminate", &ドライブ, &残り, 一時ディレクトリ.path(), rclone実行ファイル.as_deref())?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;

    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "upload", "remote": "origin"}))?;
    let 応答 = プロセス.一行受け取る()?;
    assert_eq!(応答, serde_json::json!({}));

    プロセス.一行送る(&serde_json::json!({"event": "terminate"}))?;
    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(終了状態.success(), "terminateだけの往復は正常終了であるべき");
    Ok(())
}
