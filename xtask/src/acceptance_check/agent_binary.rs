//! 対象実行ファイル（`gitlfs-tools`）のビルドと絶対パスの解決。`cargo run`は
//! 使わない。Gitに登録する`lfs.customtransfer.gitlfs-tools.path`は永続する絶対パスで
//! ある必要があり、`cargo run`が挟む一時実行経路と一致させないためである。

use std::path::{Path, PathBuf};
use std::process::Command;

const 二進表現名: &str = "gitlfs-tools";

#[derive(Clone)]
pub struct 対象実行ファイルパス(PathBuf);

impl 対象実行ファイルパス {
    /// `cargo build`で最新化してから絶対パスを返す。
    pub fn ビルドして解決する() -> Result<Self, String> {
        let cargo実行ファイル = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
        let 結果 = Command::new(&cargo実行ファイル)
            .args(["build", "--package", "gitlfs-tools-cli", "--bin", 二進表現名])
            .status()
            .map_err(|失敗| format!("cargo buildを起動できなかった: {失敗}"))?;
        if !結果.success() {
            return Err("対象実行ファイルのビルドに失敗した".to_owned());
        }

        let 現在地 = std::env::current_dir().map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))?;
        let 対象ディレクトリ名 = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_owned());
        let パス = 現在地.join(対象ディレクトリ名).join("debug").join(format!("{二進表現名}{}", std::env::consts::EXE_SUFFIX));
        if !パス.is_file() {
            return Err(format!("ビルド後も対象実行ファイルが見つからない: {}", パス.display()));
        }
        Ok(Self(パス))
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }
}
