//! initが失敗する経路を確かめる。未定義プロファイル・プロジェクト設定未検出の2つは
//! rcloneの起動確認より前に失敗するため、実rcloneが無くても実行できる。
//! 3つめ（存在しないリモート名）はrcloneの起動確認そのものを踏むため実rcloneが要り、
//! PATHまたは`GITLFS_TOOLS_TEST_EXECUTABLE`から実行ファイルを解決する。解決できなければ、
//! `GITLFS_TOOLS_SKIP_INTEGRATION`が設定されている場合に限り読み飛ばし、それ以外は失敗させる
//! （`common::rclone_executable`参照）。
//! 4つめ（実行ファイルパスが不正）はPC設定`rclone_executable`が指すファイルが実在しない
//! ため、起動確認そのものが失敗する経路であり、実rcloneが無くても実行できる。
//! PATH解決の側（PATH上にrcloneが無い場合）は、`std::env::set_var`がedition 2024で
//! `unsafe fn`になり本ワークスペースの`unsafe_code = "forbid"`と両立しないため、
//! PATHを安全に操作する手段が無く実測できない。

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
            eprintln!("GITLFS_TOOLS_SKIP_INTEGRATION が設定されているため、結合テストを読み飛ばします。");
            return Ok(());
        }
    };

    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("bad-remote")?;
    let pc設定 = common::fixtures::pc設定ディレクトリを作る(
        "bad-remote",
        "definitely-not-a-configured-remote-gitlfs-tools-storage-test",
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

#[test]
fn 実行ファイルパスが不正でinitに失敗しコード10以外を返す() -> Result<(), Box<dyn std::error::Error>> {
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("bad-executable")?;
    let 存在しない実行ファイル = 一時ディレクトリ.path().join("no-such-rclone-executable");
    let pc設定 = common::fixtures::pc設定ディレクトリを作る(
        "bad-executable",
        "dummyremote",
        "dummy",
        一時ディレクトリ.path(),
        Some(存在しない実行ファイル.as_path()),
    )?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&serde_json::json!({"event": "init", "operation": "upload", "remote": "origin"}))?;

    let 応答 = プロセス.一行受け取る()?;
    let コード = 応答["error"]["code"].as_u64();
    assert_ne!(コード, Some(10), "起動できない実行ファイルはコード10（認証接続失敗）ではないはず: {応答:?}");
    assert_eq!(コード, Some(8), "起動できない実行ファイルはコード8（rclone実行ファイル不在）になるべき: {応答:?}");

    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(!終了状態.success());
    Ok(())
}
