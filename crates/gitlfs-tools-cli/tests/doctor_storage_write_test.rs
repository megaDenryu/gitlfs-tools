//! `doctor`の追加診断4「保管先への書き込み確認」。実rcloneのlocal backendへ実際に
//! 書き込み・読み戻し・削除を行い、一時オブジェクトが残らないことをファイルシステムで
//! 確かめる。既定でPATHから`rclone`を解決し、`GITLFS_TOOLS_SKIP_INTEGRATION`が設定されて
//! いる場合に限り読み飛ばす（`common::rclone_executable`参照）。

mod common;

#[test]
fn doctorは保管先への書き込みを確認し一時オブジェクトを残さない() -> Result<(), Box<dyn std::error::Error>> {
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
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("doctor-write-check")?;
    let pc設定 = common::fixtures::pc設定ディレクトリを作る(
        "doctor-write-check",
        &ドライブ,
        &残り,
        一時ディレクトリ.path(),
        rclone実行ファイル.as_deref(),
    )?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(
        作業ツリー.path(),
        &["doctor"],
        &[("GITLFS_TOOLS_PC_CONFIG_DIR", pc設定.path().as_os_str())],
    )?;

    assert!(結果.標準出力.contains("[OK] 保管先への書き込み確認"), "書き込み確認は成功するべき: {}", 結果.標準出力);

    let 一時オブジェクト領域 = 保管先ルート.path().join("lfs").join("tmp");
    let 残存件数 = if 一時オブジェクト領域.is_dir() { std::fs::read_dir(&一時オブジェクト領域)?.count() } else { 0 };
    assert_eq!(残存件数, 0, "確認に使った一時オブジェクトを残さないべき: {一時オブジェクト領域:?}");
    Ok(())
}
