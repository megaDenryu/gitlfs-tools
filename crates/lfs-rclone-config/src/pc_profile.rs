//! 論理プロファイル名を解決した先のPCプロファイルを表すドメインモデル。

use std::time::Duration;

use lfs_rclone_domain::{Rclone実行ファイルの場所, Rcloneリモート名, 一時ディレクトリ, 保管先基底パス, 転送タイムアウト};

use crate::config_error::設定エラー;
use crate::pc_config_toml::PCプロファイルTOML表現;

/// TOMLの`transfer_timeout_seconds`（秒数）から`転送タイムアウト`を組み立てる。
/// TOML固有のフィールド形式（生の秒数）を扱うため、domain層でなくこの層に置く。
fn 秒数から転送タイムアウトを生成する(秒数: u64) -> 転送タイムアウト {
    転送タイムアウト::生成する(Duration::from_secs(秒数))
}

/// TOMLの`rclone_executable`（省略可能な文字列）から`Rclone実行ファイルの場所`を組み立てる。
/// 「未指定ならPATH解決に委ねる」というTOML固有の解釈を扱うため、domain層でなくこの層に置く。
fn 実行ファイル指定文字列から生成する(値: Option<String>) -> Rclone実行ファイルの場所 {
    match 値 {
        Some(パス文字列) => Rclone実行ファイルの場所::指定パスから生成する(パス文字列),
        None => Rclone実行ファイルの場所::解決を環境変数に委ねる(),
    }
}

/// PC設定の1プロファイルを解決した値。`lfs-rclone-rclone`（#5）と`lfs-rclone-protocol`
/// （#7）がそのまま受け取れる、名前のある型である。この層はrcloneを起動しない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PCプロファイル {
    rcloneリモート: Rcloneリモート名,
    基底パス: 保管先基底パス,
    一時ディレクトリ: 一時ディレクトリ,
    転送タイムアウト: 転送タイムアウト,
    rclone実行ファイル: Rclone実行ファイルの場所,
}

impl PCプロファイル {
    pub(crate) fn 生成する(表現: PCプロファイルTOML表現) -> Result<Self, 設定エラー> {
        let 設定不備として変換する = |エラー: lfs_rclone_domain::保管エラー| 設定エラー::解析失敗 {
            説明: エラー.to_string(),
        };

        let rcloneリモート = Rcloneリモート名::生成する(表現.rclone_remote).map_err(設定不備として変換する)?;
        let 基底パス = 保管先基底パス::生成する(表現.base_path).map_err(設定不備として変換する)?;
        let 一時ディレクトリ = 一時ディレクトリ::生成する(表現.temp_directory);
        let 転送タイムアウト = 秒数から転送タイムアウトを生成する(表現.transfer_timeout_seconds);
        let rclone実行ファイル = 実行ファイル指定文字列から生成する(表現.rclone_executable);

        Ok(Self {
            rcloneリモート,
            基底パス,
            一時ディレクトリ,
            転送タイムアウト,
            rclone実行ファイル,
        })
    }

    pub fn rcloneリモート(&self) -> &Rcloneリモート名 {
        &self.rcloneリモート
    }

    pub fn 基底パス(&self) -> &保管先基底パス {
        &self.基底パス
    }

    pub fn 一時ディレクトリ(&self) -> &一時ディレクトリ {
        &self.一時ディレクトリ
    }

    pub fn 転送タイムアウト(&self) -> 転送タイムアウト {
        self.転送タイムアウト
    }

    pub fn rclone実行ファイル(&self) -> &Rclone実行ファイルの場所 {
        &self.rclone実行ファイル
    }
}
