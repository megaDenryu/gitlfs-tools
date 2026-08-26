//! PC設定ファイルの探索と読み込みを行う外部境界。
//!
//! 役割は「外部境界」（コード分割規約.md 1節）。PC設定はGit作業ツリーの外、
//! WindowsではユーザーAppData、他OSでは対応するXDG設定ディレクトリに置く。

use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::config_error::設定エラー;
use crate::pc_config::PC設定;
use crate::pc_config_toml::PC設定TOML表現;

const アプリケーション識別子: &str = "git-lfs-rclone-storage";
const PC設定ファイル名: &str = "config.toml";

/// PC設定ファイルの置き場所。パスの綴りをこの型のメソッドへ閉じ、呼び出し側で`join`を
/// 連ねさせない（グローバルCLAUDE.md「役割の型は自分の配置を知る」）。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct PC設定の場所(PathBuf);

impl PC設定の場所 {
    /// OS標準の設定ディレクトリを`directories`crateで解決する。
    pub fn 既定の場所を解決する() -> Result<Self, 設定エラー> {
        let プロジェクトディレクトリ =
            ProjectDirs::from("", "", アプリケーション識別子).ok_or(設定エラー::PC設定ディレクトリ不明)?;

        Ok(Self(プロジェクトディレクトリ.config_dir().join(PC設定ファイル名)))
    }

    /// テスト専用の入口。実ユーザーの設定ディレクトリを読み書きするテストを避けるため、
    /// 任意のディレクトリをPC設定の置き場所として扱う。
    pub fn ディレクトリを指定して生成する(ディレクトリ: impl Into<PathBuf>) -> Self {
        Self(ディレクトリ.into().join(PC設定ファイル名))
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// PC設定を読み込み、解析・スキーマ検証する。ファイルが存在しない場合は
    /// `設定エラー::PC設定未検出`を返し、解析に失敗する場合と区別する。
    pub fn 読み込む(&self) -> Result<PC設定, 設定エラー> {
        if !self.0.is_file() {
            return Err(設定エラー::PC設定未検出);
        }

        let 本文 = fs::read_to_string(&self.0).map_err(|エラー| 設定エラー::ローカル入出力 {
            説明: format!("PC設定の読み込みに失敗しました: {エラー}"),
        })?;

        let 表現: PC設定TOML表現 =
            toml::from_str(&本文).map_err(|エラー| 設定エラー::解析失敗 { 説明: エラー.message().to_owned() })?;

        PC設定::生成する(表現)
    }
}
