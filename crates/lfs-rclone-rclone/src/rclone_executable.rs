//! rclone実行ファイルの指定方法を表す値型。
//!
//! 明示パスとPATH解決を判別共用体で区別する。「無し」を空文字や`Option`へ密輸せず、
//! どちらの解決方法を使うかを型そのもので表す
//! （グローバルCLAUDE.md「『不在・未設定・使用不可』状態の設計」）。
//!
//! 注意: `PATH解決`の「PATH」はOSの環境変数`PATH`そのものを指す外部仕様の名前であり、
//! 独自に翻訳しない。

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

const PATH解決時のコマンド名: &str = "rclone";

#[derive(Debug, Clone, PartialEq, Eq)]
enum 実行ファイル位置 {
    指定パス(PathBuf),
    PATH解決,
}

/// rclone実行ファイルの指定。構築時にどちらか一方の解決方法を選ぶ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rclone実行ファイル(実行ファイル位置);

impl Rclone実行ファイル {
    /// 実行ファイルの絶対パスまたは相対パスを明示する。
    pub fn 指定パスから生成する(パス: impl Into<PathBuf>) -> Self {
        Self(実行ファイル位置::指定パス(パス.into()))
    }

    /// OSのPATH環境変数からの解決に委ねる。
    pub fn 解決を環境変数に委ねる() -> Self {
        Self(実行ファイル位置::PATH解決)
    }

    /// この指定に従って起動準備済みの`Command`を作る。引数はまだ積んでいない。
    pub(crate) fn コマンドを生成する(&self) -> Command {
        let プログラム名: &OsStr = match &self.0 {
            実行ファイル位置::指定パス(パス) => パス.as_os_str(),
            実行ファイル位置::PATH解決 => OsStr::new(PATH解決時のコマンド名),
        };
        Command::new(プログラム名)
    }
}
