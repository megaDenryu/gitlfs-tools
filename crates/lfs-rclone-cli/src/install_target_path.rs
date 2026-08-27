//! `install`が対象リポジトリのGit設定へ書き込む実行ファイルパスを表す値型。既定は
//! `std::env::current_exe`が返す、現在動作中の実行ファイル自身の絶対パスであり、
//! `--path`引数で上書きできる（グローバルCLAUDE.md「プリミティブ執着禁止はパス・
//! テキスト・名前にも適用する」。裸の`PathBuf`をコマンド実行の引数・戻り値に出さない）。

use std::env;
use std::path::PathBuf;

use crate::command_error::コマンド実行エラー;

#[repr(transparent)]
pub(crate) struct 登録する実行ファイルパス(PathBuf);

impl 登録する実行ファイルパス {
    pub(crate) fn 現在の実行ファイルから生成する() -> Result<Self, コマンド実行エラー> {
        env::current_exe().map(Self).map_err(|エラー| コマンド実行エラー::実行ファイルパス取得失敗 { 説明: エラー.to_string() })
    }

    pub(crate) fn 指定パスから生成する(パス: impl Into<PathBuf>) -> Self {
        Self(パス.into())
    }

    /// Git設定へ書き込むための文字列表現（境界1箇所）。
    pub(crate) fn 設定値の文字列表現(&self) -> String {
        self.0.display().to_string()
    }
}
