//! `doctor`のGit LFSフィルター登録確認（`git lfs install --local`を実行済みか）。
//! 実rcloneやPC設定を必要としないため、隔離したGitリポジトリだけで検査できる。

mod common;

#[test]
fn doctorはgit_lfsフィルターが未登録なら不足を報告する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains("[不足] このリポジトリでのGit LFSフィルター登録確認"), "stdout={}", 結果.標準出力);
    Ok(())
}

#[test]
fn doctorはgit_lfsフィルターが登録済みなら問題なしとする() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;
    common::git_fixture::lfsを有効化する(作業ディレクトリ.path())?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains("[OK] このリポジトリでのGit LFSフィルター登録確認"), "stdout={}", 結果.標準出力);
    Ok(())
}
