//! Git作業ツリーがcommitするプロジェクト設定を表すドメインモデル。

use lfs_rclone_domain::プロファイル名;

use crate::config_error::設定エラー;
use crate::config_schema_version::設定スキーマ版;
use crate::project_config_toml::プロジェクト設定TOML表現;

/// `.large-assets.toml`から読み取ったプロジェクト設定。schema版と論理プロファイル名の
/// 2項目だけを持つ（Issue #4「プロジェクト設定」節）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct プロジェクト設定 {
    スキーマ版: 設定スキーマ版,
    プロファイル: プロファイル名,
}

impl プロジェクト設定 {
    pub(crate) fn 生成する(表現: プロジェクト設定TOML表現) -> Result<Self, 設定エラー> {
        let スキーマ版 = 設定スキーマ版::生成する(表現.schema_version)?;
        let プロファイル =
            プロファイル名::生成する(表現.profile).map_err(|エラー| 設定エラー::解析失敗 { 説明: エラー.to_string() })?;

        Ok(Self { スキーマ版, プロファイル })
    }

    pub fn スキーマ版(&self) -> 設定スキーマ版 {
        self.スキーマ版
    }

    /// PC設定で解決する論理プロファイル名。
    pub fn プロファイル(&self) -> &プロファイル名 {
        &self.プロファイル
    }
}
