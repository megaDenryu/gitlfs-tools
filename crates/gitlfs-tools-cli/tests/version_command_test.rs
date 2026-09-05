//! `version`が版を1行で出すこと、および`doctor`の先頭行が同じ版を出すことを確かめる。
//! 2台のPCへ同じ版を入れたかどうかを利用者がこの2つで見比べるため、両方が同じ値を出す
//! ことが機能である。

mod common;

#[test]
fn versionは実行ファイルの版を1行で表示して正常終了する() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 結果 = common::cli_invocation::サブコマンドを実行する(ディレクトリ.path(), &["version"], &[])?;
    assert!(結果.成功したか, "versionは正常終了するべき: {}", 結果.標準エラー出力);

    let 期待する1行 = format!("gitlfs-tools {}", env!("CARGO_PKG_VERSION"));
    assert_eq!(結果.標準出力.trim_end(), 期待する1行);
    Ok(())
}

#[test]
fn doctorの先頭行はversionと同じ版を情報として示す() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 結果 = common::cli_invocation::サブコマンドを実行する(ディレクトリ.path(), &["doctor"], &[])?;

    let 先頭行 = 結果.標準出力.lines().next().unwrap_or_default();
    let 期待する先頭行 = format!("[情報] gitlfs-tools の版: {}", env!("CARGO_PKG_VERSION"));
    assert_eq!(先頭行, 期待する先頭行, "doctorの出力全体: {}", 結果.標準出力);
    Ok(())
}
