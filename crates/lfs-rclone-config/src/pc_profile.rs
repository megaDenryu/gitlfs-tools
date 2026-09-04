//! 論理プロファイル名を解決した先のPCプロファイルを表すドメインモデル。

use lfs_rclone_domain::保管先基底パス;

use crate::config_error::設定エラー;
use crate::deprecated_setting::使われなくなった設定項目;
use crate::pc_config_toml::PCプロファイルTOML表現;
use crate::storage_specification::保管先の指定;

/// PC設定の1プロファイルを解決した値。`lfs-rclone-rclone`・`lfs-rclone-local`（保管庫の実装）
/// と`lfs-rclone-protocol`がそのまま受け取れる、名前のある型である。この層は保管先へ触らない
/// （rcloneを起動せず、ファイルシステムの状態も見ない）。
///
/// ローカルの一時ファイルの置き場所は持たない。ダウンロードの一時ファイルはGit LFSが
/// renameで奪うため、置き場所はPC設定ではなくリポジトリ側から決まる
/// （`lfs-rclone-cli`の`git_lfs_storage_directory.rs`）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PCプロファイル {
    保管先: 保管先の指定,
    基底パス: 保管先基底パス,
    使われなくなった項目一覧: Vec<使われなくなった設定項目>,
}

impl PCプロファイル {
    pub(crate) fn 生成する(表現: PCプロファイルTOML表現) -> Result<Self, 設定エラー> {
        let 保管先 = 保管先の指定::表現から生成する(&表現)?;
        let 基底パス = 保管先基底パス::生成する(表現.base_path).map_err(|エラー| 設定エラー::解析失敗 {
            説明: エラー.to_string(),
        })?;
        let mut 使われなくなった項目一覧 = Vec::new();
        if 表現.temp_directory.is_some() {
            使われなくなった項目一覧.push(使われなくなった設定項目::一時ディレクトリの指定);
        }

        Ok(Self { 保管先, 基底パス, 使われなくなった項目一覧 })
    }

    /// どの方式で保管するかと、その方式だけが使う設定値。
    pub fn 保管先(&self) -> &保管先の指定 {
        &self.保管先
    }

    pub fn 基底パス(&self) -> &保管先基底パス {
        &self.基底パス
    }

    /// このプロファイルに書かれていたが、agentが読まなかった項目。`doctor`が利用者へ
    /// 「消してよい」と伝えるために使う。
    pub fn 使われなくなった項目一覧(&self) -> &[使われなくなった設定項目] {
        &self.使われなくなった項目一覧
    }
}
