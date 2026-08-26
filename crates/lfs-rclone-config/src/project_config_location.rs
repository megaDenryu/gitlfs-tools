//! `.large-assets.toml`の探索と読み込みを行う外部境界。
//!
//! 役割は「外部境界」（コード分割規約.md 1節）。Git LFSはリポジトリ内の任意のディレクトリから
//! agentを起動しうるため、起点ディレクトリから親をたどって`.large-assets.toml`を探す。

use std::fs;
use std::path::{Path, PathBuf};

use crate::config_error::設定エラー;
use crate::project_config::プロジェクト設定;
use crate::project_config_toml::プロジェクト設定TOML表現;

const プロジェクト設定ファイル名: &str = ".large-assets.toml";

/// 探索で見つかった`.large-assets.toml`の場所。パスの綴りをこの型のメソッドへ閉じ、
/// 呼び出し側で`join`を連ねさせない（グローバルCLAUDE.md「役割の型は自分の配置を知る」）。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct プロジェクト設定の場所(PathBuf);

impl プロジェクト設定の場所 {
    /// `起点`から親ディレクトリを順にたどり、`.large-assets.toml`を探す。
    /// 起点はテストで一時ディレクトリを指定できるよう引数で受け取る。
    pub fn 探索する(起点: &Path) -> Result<Self, 設定エラー> {
        let mut 現在地 = Some(起点);
        while let Some(ディレクトリ) = 現在地 {
            let 候補パス = ディレクトリ.join(プロジェクト設定ファイル名);
            if 候補パス.is_file() {
                return Ok(Self(候補パス));
            }
            現在地 = ディレクトリ.parent();
        }
        Err(設定エラー::プロジェクト設定未検出)
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// 発見済みの設定ファイルを読み込み、解析・スキーマ検証する。
    pub fn 読み込む(&self) -> Result<プロジェクト設定, 設定エラー> {
        let 本文 = fs::read_to_string(&self.0).map_err(|エラー| 設定エラー::ローカル入出力 {
            説明: format!("プロジェクト設定の読み込みに失敗しました: {エラー}"),
        })?;

        let 表現: プロジェクト設定TOML表現 =
            toml::from_str(&本文).map_err(|エラー| 設定エラー::解析失敗 { 説明: エラー.message().to_owned() })?;

        プロジェクト設定::生成する(表現)
    }
}
