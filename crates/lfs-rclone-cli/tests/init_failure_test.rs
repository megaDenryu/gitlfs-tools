//! initが失敗する経路を確かめる。未定義プロファイル・プロジェクト設定未検出の2つは
//! rcloneの起動確認より前に失敗するため、実rcloneが無くても実行できる。
//! 3つめ（存在しないリモート名）はrcloneの起動確認そのものを踏むため実rcloneが要り、
//! PATHまたは`LFS_RCLONE_TEST_EXECUTABLE`から実行ファイルを解決する。解決できなければ、
//! `LFS_RCLONE_SKIP_INTEGRATION`が設定されている場合に限り読み飛ばし、それ以外は失敗させる
//! （`common::rclone_executable`参照）。

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

#[test]
fn 存在しないリモート名でinitに失敗しコード10を返す() -> Result<(), Box<dyn std::error::Error>> {
    let rclone実行ファイル = match common::rclone_executable::実行ファイルを解決する()? {
        common::rclone_executable::実行ファイル解決::明示された場所(パス) => Some(パス),
        common::rclone_executable::実行ファイル解決::PATH解決 => None,
        common::rclone_executable::実行ファイル解決::読み飛ばす => {
            eprintln!("LFS_RCLONE_SKIP_INTEGRATION が設定されているため、結合テストを読み飛ばします。");
            return Ok(());
        }
    };

    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("bad-remote")?;
    let pc設定 = common::fixtures::pc設定ディレクトリを作る(
        "bad-remote",
        "definitely-not-a-configured-remote-lfs-rclone-storage-test",
        "somepath",
        一時ディレクトリ.path(),
        rclone実行ファイル.as_deref(),
    )?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "upload", "remote": "origin"}))?;

    let 応答 = プロセス.一行受け取る()?;
    let コード = 応答["error"]["code"].as_u64();
    assert_eq!(コード, Some(10), "存在しないリモート名はコード10（認証接続失敗）になるべき: {応答:?}");

    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(!終了状態.success());
    Ok(())
}
