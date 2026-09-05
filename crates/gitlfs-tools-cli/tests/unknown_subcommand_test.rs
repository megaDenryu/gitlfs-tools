//! 未知のサブコマンドは、黙ってプロトコル通信へ進まず、明確な失敗として終わることを
//! 確かめる（引数なしの起動だけがプロトコル通信であるべき。CLAUDE.md「大前提」）。

mod common;

#[test]
fn 未知のサブコマンドは非0終了で使い方を示す() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 結果 = common::cli_invocation::サブコマンドを実行する(ディレクトリ.path(), &["no-such-subcommand"], &[])?;
    assert!(!結果.成功したか, "未知のサブコマンドは非0終了するべき");
    assert!(結果.標準エラー出力.contains("未知のサブコマンド"), "理由を示すべき: {}", 結果.標準エラー出力);
    assert!(結果.標準エラー出力.contains("使い方"), "使い方を示すべき: {}", 結果.標準エラー出力);
    Ok(())
}

#[test]
fn helpは使い方を表示して正常終了する() -> Result<(), Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 結果 = common::cli_invocation::サブコマンドを実行する(ディレクトリ.path(), &["help"], &[])?;
    assert!(結果.成功したか);
    assert!(結果.標準出力.contains("使い方"));
    Ok(())
}
