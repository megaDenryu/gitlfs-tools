//! initが失敗する2つの経路（未定義プロファイル・プロジェクト設定未検出）を確かめる。
//! どちらもrcloneの起動確認より前に失敗するため、実rcloneが無くても実行できる。

mod common;

use std::path::Path;

#[test]
fn 未定義プロファイルでinitに失敗し非0終了する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("profile-in-project")?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let pc設定 =
        common::fixtures::pc設定ディレクトリを作る("profile-in-pc-config-only", "dummyremote", "dummy", 一時ディレクトリ.path(), None)?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "upload", "remote": "origin"}))?;

    let 応答 = プロセス.一行受け取る()?;
    assert!(応答.get("error").is_some(), "init失敗はerrorを含むべき: {応答:?}");
    assert!(応答.get("event").is_none(), "init応答はeventフィールドを持たない");

    let (終了状態, 標準エラー出力) = プロセス.終了を待って後始末する()?;
    assert!(!終了状態.success(), "init失敗は非0終了であるべき");
    assert!(標準エラー出力.contains("initに失敗しました"), "標準エラー出力に詳細が残っているべき: {標準エラー出力}");
    Ok(())
}

#[test]
fn プロジェクト設定未検出でinitに失敗し非0終了する() -> Result<(), Box<dyn std::error::Error>> {
    let 作業ツリー = tempfile::tempdir()?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let pc設定 = common::fixtures::pc設定ディレクトリを作る("any-profile", "dummyremote", "dummy", 一時ディレクトリ.path(), None)?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "download", "remote": "origin"}))?;

    let 応答 = プロセス.一行受け取る()?;
    assert!(応答.get("error").is_some(), "init失敗はerrorを含むべき: {応答:?}");

    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(!終了状態.success());
    Ok(())
}
