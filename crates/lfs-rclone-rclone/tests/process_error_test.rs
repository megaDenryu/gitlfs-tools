//! 子プロセスの起動失敗・非0終了・タイムアウトが別のエラーとして区別できること、
//! 診断メッセージへ秘密情報が混入しないこと、標準出力が捕捉されて戻り値にだけ
//! 現れることを確かめる。

mod common;

use std::time::{Duration, Instant};

use lfs_rclone_domain::{
    一時ディレクトリ, オブジェクト状態, オブジェクト識別子, 保管エラー, 保管先基底パス, 期待バイト数, Rclone実行ファイルの場所, Rcloneリモート名,
    転送タイムアウト,
};
use lfs_rclone_rclone::Rclone保管庫;
use lfs_rclone_storage_port::オブジェクト保管庫;

fn ダミー識別子() -> Result<オブジェクト識別子, Box<dyn std::error::Error>> {
    Ok(オブジェクト識別子::生成する(&"c".repeat(64))?)
}

fn 子プロセスエラーの説明を取り出す(結果: Result<オブジェクト状態, 保管エラー>) -> Result<String, Box<dyn std::error::Error>> {
    match 結果 {
        Err(保管エラー::子プロセス { 説明 }) => Ok(説明),
        他 => Err(format!("保管エラー::子プロセスを期待したが{他:?}だった").into()),
    }
}

#[test]
fn 存在しない実行ファイルなら起動失敗として区別できる() -> Result<(), Box<dyn std::error::Error>> {
    let 実行ファイル = Rclone実行ファイルの場所::指定パスから生成する("this-executable-should-not-exist-issue5.exe");
    let リモート名 = Rcloneリモート名::生成する("fakeremote")?;
    let 基底パス = 保管先基底パス::生成する("does-not-matter")?;
    let 一時ディレクトリ = 一時ディレクトリ::生成する(std::env::temp_dir());
    let 保管庫 = Rclone保管庫::生成する(
        実行ファイル,
        リモート名,
        基底パス,
        一時ディレクトリ,
        転送タイムアウト::生成する(Duration::from_secs(2)),
    );

    let 結果 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(1));
    let 説明 = 子プロセスエラーの説明を取り出す(結果)?;

    assert!(説明.contains("起動"), "起動失敗と分かる説明文であるべき: {説明}");
    Ok(())
}

#[test]
fn 非0終了は起動失敗やタイムアウトと区別でき秘密を伏せる() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("process-nonzero")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.即終了で応答させる(6, "", "SECRET_TOKEN_ABC123 must never leak into diagnostics")?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 結果 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(1));
    let 説明 = 子プロセスエラーの説明を取り出す(結果)?;

    assert!(説明.contains('6'), "終了コードが説明文に残っているべき: {説明}");
    assert!(!説明.contains("SECRET_TOKEN_ABC123"), "秘密情報が説明文へ漏れている: {説明}");
    assert!(!説明.contains("must never leak"), "標準エラー出力の内容が説明文へ漏れている: {説明}");
    Ok(())
}

#[test]
fn 応答が遅ければタイムアウトとして区別できる() -> Result<(), Box<dyn std::error::Error>> {
    let 基底パス = common::固有の基底パス文字列を作る("process-timeout")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.応答前に眠らせる(Duration::from_secs(5))?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_millis(200))?;

    let 開始時刻 = Instant::now();
    let 結果 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(1));
    let 経過時間 = 開始時刻.elapsed();
    let 説明 = 子プロセスエラーの説明を取り出す(結果)?;

    assert!(説明.contains("タイムアウト"), "タイムアウトと分かる説明文であるべき: {説明}");
    assert!(経過時間 < Duration::from_secs(3), "タイムアウト検出が遅すぎる: {経過時間:?}");
    Ok(())
}

#[test]
fn 子プロセスの標準出力はpipeで捕捉されアダプタの戻り値だけに現れる() -> Result<(), Box<dyn std::error::Error>> {
    // 注意: `Rcloneプロセス実行器`が子の標準出力を`Stdio::inherit()`にしていたら、
    // `child.stdout.take()`が`None`になり呼び出しは起動失敗エラーへ落ちる（本テストは
    // 失敗する）。加えて子の出力はこのテストプロセス自身の標準出力へそのまま漏れて
    // 混ざる。ここでは戻り値の内容が期待どおりに得られることをもって、pipeでの捕捉が
    // 効いていることの間接証拠とする。
    let 基底パス = common::固有の基底パス文字列を作る("process-stdout-capture")?;
    let 指示 = common::偽rclone指示置き場::準備する(&基底パス)?;
    指示.既存として仕込む(4096)?;
    let 保管庫 = common::偽rclone保管庫を作る(&基底パス, Duration::from_secs(5))?;

    let 状態 = 保管庫.存在を確認する(&ダミー識別子()?, 期待バイト数::生成する(4096))?;

    assert_eq!(状態, オブジェクト状態::存在);
    Ok(())
}
