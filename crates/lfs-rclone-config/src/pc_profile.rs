//! 論理プロファイル名を解決した先のPCプロファイルを表すドメインモデル。

use lfs_rclone_domain::{一時ディレクトリ, 保管先基底パス};

use crate::config_error::設定エラー;
use crate::pc_config_toml::PCプロファイルTOML表現;
use crate::storage_specification::保管先の指定;

/// PC設定の1プロファイルを解決した値。`lfs-rclone-rclone`・`lfs-rclone-local`（保管庫の実装）
/// と`lfs-rclone-protocol`がそのまま受け取れる、名前のある型である。この層は保管先へ触らない
/// （rcloneを起動せず、ファイルシステムの状態も見ない）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PCプロファイル {
    保管先: 保管先の指定,
    基底パス: 保管先基底パス,
    一時ディレクトリ: 一時ディレクトリ,
}

impl PCプロファイル {
    pub(crate) fn 生成する(表現: PCプロファイルTOML表現) -> Result<Self, 設定エラー> {
        let 保管先 = 保管先の指定::表現から生成する(&表現)?;
        let 基底パス = 保管先基底パス::生成する(表現.base_path).map_err(|エラー| 設定エラー::解析失敗 {
            説明: エラー.to_string(),
        })?;
        let 一時ディレクトリ = 一時ディレクトリ::生成する(表現.temp_directory);

        Ok(Self { 保管先, 基底パス, 一時ディレクトリ })
    }

    /// どの方式で保管するかと、その方式だけが使う設定値。
    pub fn 保管先(&self) -> &保管先の指定 {
        &self.保管先
    }

    pub fn 基底パス(&self) -> &保管先基底パス {
        &self.基底パス
    }

    pub fn 一時ディレクトリ(&self) -> &一時ディレクトリ {
        &self.一時ディレクトリ
    }
}
