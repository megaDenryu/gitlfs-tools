//! init成功後、0件の転送要求のままterminateしたときの往復を確かめる。
//! 実行ファイルの場所は環境変数`LFS_RCLONE_TEST_EXECUTABLE`で受け取る。未設定、または
//! 指すファイルが存在しなければ、テストを失敗させずに読み飛ばした旨を標準エラー出力へ
//! 書いて成功扱いで終える（`lfs-rclone-rclone`の結合テストと同じ方針）。

mod common;

use std::path::Path;

#[test]
fn init成功後に0件転送でterminateすると正常終了する() -> Result<(), Box<dyn std::error::Error>> {
    let Some(rclone実行ファイル) = common::fixtures::rclone実行ファイルのパスを探す() else {
        eprintln!("LFS_RCLONE_TEST_EXECUTABLE が未設定、または指すファイルが見つからないため、結合テストを読み飛ばします。");
        return Ok(());
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = common::fixtures::ドライブとパスへ分ける(保管先ルート.path())?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("empty-terminate")?;
    let pc設定 =
        common::fixtures::pc設定ディレクトリを作る("empty-terminate", &ドライブ, &残り, 一時ディレクトリ.path(), Some(&rclone実行ファイル))?;

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
