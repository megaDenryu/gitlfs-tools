//! 実rcloneとの結合テストが使う実行ファイルの解決。既定でPATHから`rclone`を解決し、
//! 見つからない場合はテストを黙って通過させず失敗させる。読み飛ばしは
//! `LFS_RCLONE_SKIP_INTEGRATION`が設定されているときだけ明示的に行う
//! （`lfs-rclone-cli`の同名モジュールと同じ方針。crateをまたぐため実装は複製する）。
//!
//! 注意: PATH解決の場合は明示パスを持たない。ここで`Command::new("rclone")`が
//! 起動できることを確かめるだけであり、絶対パスへは変換しない。呼び出し側は
//! `Rclone実行ファイルの場所::解決を環境変数に委ねる()`を使う。

use std::path::PathBuf;
use std::process::Command;

/// 実rcloneとの結合テストが使う実行ファイルの解決結果。
pub enum 実行ファイル解決 {
    明示された場所(PathBuf),
    PATH解決,
    読み飛ばす,
}

/// `LFS_RCLONE_TEST_EXECUTABLE`が指定されていればそれを使う。未指定ならPATHから
/// `rclone`を解決する。どちらも失敗したらエラーを返し、テストを黙って通過させない。
/// `LFS_RCLONE_SKIP_INTEGRATION`が設定されている場合に限り、明示的に読み飛ばす。
pub fn 実行ファイルを解決する() -> Result<実行ファイル解決, String> {
    if std::env::var("LFS_RCLONE_SKIP_INTEGRATION").is_ok() {
        return Ok(実行ファイル解決::読み飛ばす);
    }
    if let Ok(指定パス文字列) = std::env::var("LFS_RCLONE_TEST_EXECUTABLE") {
        return 指定パスを検査する(指定パス文字列);
    }
    if パスから起動できるか() {
        return Ok(実行ファイル解決::PATH解決);
    }
    Err(
        "rclone が見つかりません。PATH へ導入するか、LFS_RCLONE_TEST_EXECUTABLE で場所を指定するか、\
         LFS_RCLONE_SKIP_INTEGRATION を設定して読み飛ばしてください。"
            .to_owned(),
    )
}

fn 指定パスを検査する(指定パス文字列: String) -> Result<実行ファイル解決, String> {
    let パス = PathBuf::from(&指定パス文字列);
    if パス.is_file() {
        Ok(実行ファイル解決::明示された場所(パス))
    } else {
        Err(format!("LFS_RCLONE_TEST_EXECUTABLE が指すファイルが見つかりません: {指定パス文字列}"))
    }
}

fn パスから起動できるか() -> bool {
    Command::new("rclone").arg("version").output().map(|出力| 出力.status.success()).unwrap_or(false)
}
