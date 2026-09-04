//! releaseプロファイルでのビルドと、生成された実行ファイルの絶対パスの解決。
//!
//! 日常使う実行ファイルが開発ビルドであってはならないため、配置の前段は必ず`--release`で行う。
//! `cargo run`は使わない。配置してGit設定へ登録する対象は永続する実体のファイルである。

use std::path::{Path, PathBuf};
use std::process::Command;

const 実行ファイルの語幹: &str = "git-lfs-rclone-storage";

/// 配置元となる、releaseプロファイルでビルドされた実行ファイル。
pub struct リリースビルド成果物(PathBuf);

impl リリースビルド成果物 {
    pub fn ビルドして解決する() -> Result<Self, String> {
        let cargo実行ファイル = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
        let 結果 = Command::new(cargo実行ファイル)
            .args(["build", "--release", "--package", "lfs-rclone-cli", "--bin", 実行ファイルの語幹])
            .status()
            .map_err(|失敗| format!("cargo buildを起動できなかった: {失敗}"))?;
        if !結果.success() {
            return Err("releaseビルドに失敗した".to_owned());
        }

        let パス = Self::成果物のパスを組み立てる()?;
        if !パス.is_file() {
            return Err(format!("ビルド後も実行ファイルが見つからない: {}", パス.display()));
        }
        Ok(Self(パス))
    }

    fn 成果物のパスを組み立てる() -> Result<PathBuf, String> {
        let 現在地 = std::env::current_dir().map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))?;
        let 対象ディレクトリ名 = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_owned());
        Ok(現在地.join(対象ディレクトリ名).join("release").join(実行ファイル名を組み立てる()))
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }
}

/// OSごとの実行ファイル拡張子を付けたファイル名。配置元と配置先で同じ綴りを使う。
pub fn 実行ファイル名を組み立てる() -> String {
    format!("{実行ファイルの語幹}{}", std::env::consts::EXE_SUFFIX)
}
