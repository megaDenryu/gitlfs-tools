//! ローカルディレクトリ方式（PC設定`storage = "local"`）での`doctor`。rcloneの起動確認では
//! なく保管先ルートディレクトリの存在確認を出すこと、書き込み確認が成功して一時オブジェクトを
//! 残さないこと、基点が無いときに何をすればよいかを示すことを確かめる。rcloneに依存しない
//! ため、この結合テストは環境によらず既定で実行する。

mod common;

use std::ffi::OsStr;
use std::path::Path;

fn doctorを実行する(
    作業ツリー: &Path,
    pc設定ディレクトリ: &Path,
) -> Result<common::cli_invocation::サブコマンド実行結果, Box<dyn std::error::Error>> {
    common::cli_invocation::サブコマンドを実行する(
        作業ツリー,
        &["doctor"],
        &[("LFS_RCLONE_PC_CONFIG_DIR", OsStr::new(pc設定ディレクトリ))],
    )
}

#[test]
fn 保管先ルートの存在確認と書き込み確認が成功する() -> Result<(), Box<dyn std::error::Error>> {
    let 保管先ルート = tempfile::tempdir()?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("doctor-local")?;
    let pc設定 = common::fixtures::ローカル方式のpc設定ディレクトリを作る("doctor-local", 保管先ルート.path(), 一時ディレクトリ.path())?;

    let 結果 = doctorを実行する(作業ツリー.path(), pc設定.path())?;

    assert!(
        結果.標準出力.contains("[OK] 保管先ルートディレクトリの存在確認"),
        "ローカルディレクトリ方式では基点の存在確認を出すべき: {}",
        結果.標準出力
    );
    assert!(!結果.標準出力.contains("rcloneの起動確認"), "子プロセスを使わない方式でrcloneの起動確認を出してはならない: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("[OK] 保管先への書き込み確認"), "書き込み確認は成功するべき: {}", 結果.標準出力);

    let 一時オブジェクト領域 = 保管先ルート.path().join("lfs").join("tmp");
    let 残存件数 = if 一時オブジェクト領域.is_dir() { std::fs::read_dir(&一時オブジェクト領域)?.count() } else { 0 };
    assert_eq!(残存件数, 0, "確認に使った一時オブジェクトを残さないべき");
    Ok(())
}

#[test]
fn 保管先ルートが存在しなければ対処とともに不足を報告する() -> Result<(), Box<dyn std::error::Error>> {
    let 親 = tempfile::tempdir()?;
    let 存在しない保管先ルート = 親.path().join("mounted-drive-is-absent");
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("doctor-local-absent")?;
    let pc設定 =
        common::fixtures::ローカル方式のpc設定ディレクトリを作る("doctor-local-absent", &存在しない保管先ルート, 一時ディレクトリ.path())?;

    let 結果 = doctorを実行する(作業ツリー.path(), pc設定.path())?;

    assert!(結果.標準出力.contains("[不足] 保管先ルートディレクトリの存在確認"), "不足として報告するべき: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("マウント"), "対処にマウントの確認を含めるべき: {}", 結果.標準出力);
    assert!(!結果.成功したか, "不足があるときのdoctorは失敗として終わるべき");
    Ok(())
}
