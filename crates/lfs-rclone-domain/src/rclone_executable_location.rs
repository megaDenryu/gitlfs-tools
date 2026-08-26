//! rclone実行ファイルの指定方法を表す値型。
//!
//! 明示パスとPATH解決を判別共用体で区別する。「無し」を空文字や`Option`へ密輸せず、
//! どちらの解決方法を使うかを型そのもので表す
//! （グローバルCLAUDE.md「『不在・未設定・使用不可』状態の設計」）。
//!
//! 注意: `PATH上の実行ファイル`の「PATH」はOSの環境変数`PATH`そのものを指す外部仕様の
//! 名前であり、独自に翻訳しない。子プロセスの起動（`std::process::Command`の構築）は
//! `lfs-rclone-rclone`の外部境界が担う。この型は起動に使うプログラム名を渡すだけに留め、
//! `std::process`は知らない。

use std::ffi::OsStr;
use std::path::PathBuf;

const PATH解決時のプログラム名: &str = "rclone";

/// rclone実行ファイルの指定。PC設定`rclone_executable`が明示したパスと、省略時に
/// OSのPATH環境変数へ解決を委ねる場合を区別する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rclone実行ファイルの場所 {
    明示された場所(PathBuf), // PC設定`rclone_executable`が明示したパス
    PATH上の実行ファイル,     // `rclone_executable`が省略された。PATH上の`rclone`を使う
}

impl Rclone実行ファイルの場所 {
    /// 実行ファイルの絶対パスまたは相対パスを明示する。
    pub fn 指定パスから生成する(パス: impl Into<PathBuf>) -> Self {
        Self::明示された場所(パス.into())
    }

    /// OSのPATH環境変数からの解決に委ねる。
    pub fn 解決を環境変数に委ねる() -> Self {
        Self::PATH上の実行ファイル
    }

    /// 子プロセスの起動へ渡すプログラム名。
    pub fn プログラム名(&self) -> &OsStr {
        match self {
            Self::明示された場所(パス) => パス.as_os_str(),
            Self::PATH上の実行ファイル => OsStr::new(PATH解決時のプログラム名),
        }
    }
}
