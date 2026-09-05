//! サブコマンド呼び出し（`install`・`init-project`・`doctor`・`help`等）をプロセスとして
//! 実行し、終了状態・標準出力・標準エラー出力をまとめて返すテスト専用ヘルパー。
//! プロトコル通信（`common::process`）とは別の起動経路のため別ファイルに分ける。

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub struct サブコマンド実行結果 {
    pub 成功したか: bool,
    pub 標準出力: String,
    pub 標準エラー出力: String,
}

pub fn サブコマンドを実行する(
    作業ディレクトリ: &Path,
    引数: &[&str],
    環境変数: &[(&str, &OsStr)],
) -> Result<サブコマンド実行結果, Box<dyn std::error::Error>> {
    let mut コマンド = Command::new(super::fixtures::実行ファイルのパス);
    コマンド.current_dir(作業ディレクトリ).args(引数);
    for (キー, 値) in 環境変数 {
        コマンド.env(キー, 値);
    }
    let 出力 = コマンド.output()?;
    Ok(サブコマンド実行結果 {
        成功したか: 出力.status.success(),
        標準出力: String::from_utf8_lossy(&出力.stdout).into_owned(),
        標準エラー出力: String::from_utf8_lossy(&出力.stderr).into_owned(),
    })
}
