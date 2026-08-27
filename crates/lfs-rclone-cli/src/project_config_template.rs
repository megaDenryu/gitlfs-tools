//! `.large-assets.toml`雛形の本文を組み立てる値型。プロファイル名以外の値は書けない
//! （rcloneリモート名・Google Driveのパス等はPC設定側の責務であり、プロジェクト側の
//! `.large-assets.toml`へ持ち込まない。Issue #2 7.2節）。

use lfs_rclone_config::設定スキーマ版;
use lfs_rclone_domain::プロファイル名;

pub(crate) struct プロジェクト設定雛形本文(String);

impl プロジェクト設定雛形本文 {
    pub(crate) fn 生成する(スキーマ版: 設定スキーマ版, プロファイル: &プロファイル名) -> Self {
        Self(format!("schema_version = {}\nprofile = \"{}\"\n", スキーマ版.値(), プロファイル.文字列表現()))
    }

    pub(crate) fn 文字列表現(&self) -> &str {
        &self.0
    }
}
