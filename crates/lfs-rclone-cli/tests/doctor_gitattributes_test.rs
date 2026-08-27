//! `doctor`の`.gitattributes`追跡パターン確認。実rcloneやPC設定を必要としないため、
//! 隔離したGitリポジトリだけで検査できる。

mod common;

#[test]
fn doctorはgitattributesが無ければ不足を報告する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains("[不足] .gitattributesのGit LFS追跡パターン確認"), "stdout={}", 結果.標準出力);
    Ok(())
}

#[test]
fn doctorはgitattributesに追跡パターンが無ければ不足を報告する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;
    std::fs::write(作業ディレクトリ.path().join(".gitattributes"), "* text=auto\n")?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains("[不足] .gitattributesのGit LFS追跡パターン確認"), "stdout={}", 結果.標準出力);
    Ok(())
}

#[test]
fn doctorはgitattributesに追跡パターンがあれば問題なしとする() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ディレクトリ = tempfile::tempdir()?;
    common::git_fixture::初期化する(作業ディレクトリ.path())?;
    std::fs::write(作業ディレクトリ.path().join(".gitattributes"), "*.psd filter=lfs diff=lfs merge=lfs -text\n")?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(作業ディレクトリ.path(), &["doctor"], &[])?;

    assert!(結果.標準出力.contains("[OK] .gitattributesのGit LFS追跡パターン確認"), "stdout={}", 結果.標準出力);
    Ok(())
}
