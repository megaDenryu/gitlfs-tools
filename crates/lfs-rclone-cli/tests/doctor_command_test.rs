//! `doctor`サブコマンド: 設定が不足していれば非0終了で不足を報告し、すべて揃っていれば
//! 0終了することを確かめる。「揃っている」経路は実rcloneの起動確認を踏むため、PATHまたは
//! `LFS_RCLONE_TEST_EXECUTABLE`から実行ファイルを解決できない場合は
//! `LFS_RCLONE_SKIP_INTEGRATION`が設定されている場合に限り読み飛ばす
//! （`common::rclone_executable`参照）。

mod common;

#[test]
fn doctorは設定が不足していると非0終了で不足を報告する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    let 空のpc設定ディレクトリ = tempfile::tempdir()?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(
        作業ディレクトリ.path(),
        &["doctor"],
        &[("LFS_RCLONE_PC_CONFIG_DIR", 空のpc設定ディレクトリ.path().as_os_str())],
    )?;

    assert!(!結果.成功したか, "設定が不足していれば非0終了するべき");
    assert!(結果.標準出力.contains("[不足]"), "不足を報告するべき: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("プロジェクト設定"), "プロジェクト設定の不足に言及するべき: {}", 結果.標準出力);
    Ok(())
}

#[test]
fn doctorはすべて揃っていれば0終了する() -> Result<(), Box<dyn std::error::Error>> {
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
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("doctor-full")?;
    common::git_fixture::初期化する(作業ツリー.path())?;
    let pc設定 =
        common::fixtures::pc設定ディレクトリを作る("doctor-full", &ドライブ, &残り, 一時ディレクトリ.path(), rclone実行ファイル.as_deref())?;

    let 導入結果 = common::cli_invocation::サブコマンドを実行する(作業ツリー.path(), &["install"], &[])?;
    assert!(導入結果.成功したか, "事前のinstallは成功するべき: stderr={}", 導入結果.標準エラー出力);

    let 結果 = common::cli_invocation::サブコマンドを実行する(
        作業ツリー.path(),
        &["doctor"],
        &[("LFS_RCLONE_PC_CONFIG_DIR", pc設定.path().as_os_str())],
    )?;
    assert!(結果.成功したか, "すべて揃っていればdoctorは0終了するべき: stdout={} stderr={}", 結果.標準出力, 結果.標準エラー出力);
    Ok(())
}
