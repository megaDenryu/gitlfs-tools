//! PC設定ディレクトリ（`GITLFS_TOOLS_PC_CONFIG_DIR`が指す先）を表す値型。config.tomlの
//! 書き込みをこの型のメソッドへ閉じる（グローバルCLAUDE.md「役割の型は自分の配置を知る」）。

use std::fs;
use std::path::{Path, PathBuf};

use crate::acceptance_check::object_storage_root::オブジェクト保管ルート;

#[derive(Clone)]
pub struct PC設定ディレクトリ(PathBuf);

impl PC設定ディレクトリ {
    pub fn 生成する(ディレクトリ: PathBuf) -> Self {
        Self(ディレクトリ)
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// 単一の論理プロファイルだけを持つconfig.tomlを書き込む。既存ファイルは上書きする
    /// （バックエンド差し替え試験が同じ場所へ再書き込みするため）。
    pub fn 単一プロファイルで準備する(
        &self,
        プロファイル名: &str,
        保管先: &オブジェクト保管ルート,
        一時ディレクトリ: &Path,
    ) -> Result<(), String> {
        let (ドライブ, 残り) = 保管先.ドライブと残りへ分解する()?;
        fs::create_dir_all(&self.0).map_err(|失敗| format!("{}を作成できなかった: {失敗}", self.0.display()))?;
        let 一時ディレクトリ文字列 = 一時ディレクトリ.to_string_lossy().replace('\\', "/");
        let 本文 = format!(
            "schema_version = 1\n\
             [profiles.{プロファイル名}]\n\
             rclone_remote = \"{ドライブ}\"\n\
             base_path = \"{残り}\"\n\
             temp_directory = \"{一時ディレクトリ文字列}\"\n\
             transfer_timeout_seconds = 30\n"
        );
        fs::write(self.0.join("config.toml"), 本文).map_err(|失敗| format!("config.tomlを書き込めなかった: {失敗}"))
    }
}
