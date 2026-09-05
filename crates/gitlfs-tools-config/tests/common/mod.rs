//! PC設定の結合テスト共通のフィクスチャ補助関数。
//!
//! 注意: `tests/`配下の各テストファイルは個別の結合テストバイナリとしてコンパイルされ、
//! このモジュールを毎回別々にコンパイルする。

use std::io;

/// 指定した内容の`config.toml`を持つ一時ディレクトリを作る。
pub fn pc設定ディレクトリを作る(内容: &str) -> io::Result<tempfile::TempDir> {
    let ディレクトリ = tempfile::tempdir()?;
    std::fs::write(ディレクトリ.path().join("config.toml"), 内容)?;
    Ok(ディレクトリ)
}
