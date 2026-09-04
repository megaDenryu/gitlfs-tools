//! `doctor`のGit LFSフック内容確認。既にGit LFSを別の方式で使っていたリポジトリで
//! `git lfs install --local`が止まる状態を、実rcloneやPC設定を使わずに検査できる。

mod common;

const 項目名: &str = "このリポジトリのGit LFSフックが標準の内容かの確認";

#[test]
fn doctorはフックが1つも無ければ問題なしとする() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains(&format!("[OK] {項目名}")), "stdout={}", 結果.標準出力);
    Ok(())
}

#[test]
fn doctorはgit_lfsが書いた標準のフックなら問題なしとする() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;
    common::git_fixture::lfsを有効化する(作業ディレクトリ.path())?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains(&format!("[OK] {項目名}")), "stdout={}", 結果.標準出力);
    Ok(())
}

#[test]
fn doctorは送信を止める旧フックが残っていれば不足を報告する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;
    common::git_fixture::送信を止める旧pre_pushフックを置く(作業ディレクトリ.path())?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains(&format!("[不足] {項目名}")), "stdout={}", 結果.標準出力);
    assert!(結果.標準出力.contains("フックpre-pushにGit LFS以外の内容が混ざっています"), "stdout={}", 結果.標準出力);
    assert!(結果.標準出力.contains("git lfs update --force"), "stdout={}", 結果.標準出力);
    Ok(())
}
