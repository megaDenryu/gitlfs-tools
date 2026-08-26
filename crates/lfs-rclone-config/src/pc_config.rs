//! Git作業ツリーの外に置くPCごとの設定を表すドメインモデル。

use std::collections::HashMap;

use lfs_rclone_domain::プロファイル名;

use crate::config_error::設定エラー;
use crate::config_schema_version::設定スキーマ版;
use crate::pc_config_toml::PC設定TOML表現;
use crate::pc_profile::PCプロファイル;

/// PCごとの設定。論理プロファイル名から`PCプロファイル`へ解決する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PC設定 {
    スキーマ版: 設定スキーマ版,
    プロファイル一覧: HashMap<プロファイル名, PCプロファイル>,
}

impl PC設定 {
    pub(crate) fn 生成する(表現: PC設定TOML表現) -> Result<Self, 設定エラー> {
        let スキーマ版 = 設定スキーマ版::生成する(表現.schema_version)?;

        let プロファイル一覧 = 表現
            .profiles
            .into_iter()
            .map(|(名前, プロファイル表現)| {
                let プロファイル名 =
                    プロファイル名::生成する(名前).map_err(|エラー| 設定エラー::解析失敗 { 説明: エラー.to_string() })?;
                let プロファイル = PCプロファイル::生成する(プロファイル表現)?;
                Ok((プロファイル名, プロファイル))
            })
            .collect::<Result<HashMap<_, _>, 設定エラー>>()?;

        Ok(Self { スキーマ版, プロファイル一覧 })
    }

    pub fn スキーマ版(&self) -> 設定スキーマ版 {
        self.スキーマ版
    }

    /// プロジェクト設定の論理プロファイル名をPC設定へ解決する。未定義の場合は
    /// 不足しているプロファイル名だけを含む`設定エラー::未定義プロファイル`を返し、
    /// 設定の全量・PC固有の絶対パスは含めない。
    pub fn プロファイルを解決する(&self, プロファイル: &プロファイル名) -> Result<&PCプロファイル, 設定エラー> {
        self.プロファイル一覧
            .get(プロファイル)
            .ok_or_else(|| 設定エラー::未定義プロファイル {
                プロファイル名: プロファイル.clone(),
            })
    }
}
