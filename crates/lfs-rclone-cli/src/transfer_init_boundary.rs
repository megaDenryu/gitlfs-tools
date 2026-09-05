//! `転送セッション開始境界`の実装。設定読み込み・プロファイル解決・保管先の起動確認・
//! 一時保存先の作成・資産転送サービスの組み立てを行う（アーキテクチャ.md 判断2
//! 「プロトコルアダプタは保管庫の実装を知らない」）。
//!
//! 設定の3段の読み込みは`profile_resolution`が、どの方式の保管庫を組み立てるかは
//! `storage_assembly`が、その方式にどの起動確認が意味を持つかは`選択された保管庫`が決める。

use std::path::PathBuf;

use lfs_rclone_config::PC設定の場所;
use lfs_rclone_domain::{一時ディレクトリ, 保管エラー};
use lfs_rclone_protocol::{初期化エラー, 転送セッション開始境界, 転送操作種別};
use lfs_rclone_transfer::資産転送サービス;

use crate::config_error_mapping::設定エラーへ変換する;
use crate::git_lfs_storage_directory::GitLFS保管ディレクトリ;
use crate::profile_resolution::プロファイル解決に使う設定の置き場所;
use crate::storage_assembly::起動確認を済ませた保管庫を組み立てる;
use crate::temp_directory_provisioning::一時保存先を作成する;
use crate::transfer_session::転送セッション;

/// `init`要求の起点。プロジェクト設定の探索起点ディレクトリとPC設定の置き場所を保持する。
pub struct 転送セッション初期化境界 {
    設定の置き場所: プロファイル解決に使う設定の置き場所,
}

impl 転送セッション初期化境界 {
    pub fn 生成する(起点ディレクトリ: impl Into<PathBuf>, pc設定の場所: PC設定の場所) -> Self {
        Self { 設定の置き場所: プロファイル解決に使う設定の置き場所::生成する(起点ディレクトリ, pc設定の場所) }
    }
}

impl 転送セッション開始境界 for 転送セッション初期化境界 {
    type 開始済み転送セッション = 転送セッション;

    fn 開始する(&self, 操作種別: 転送操作種別) -> Result<Self::開始済み転送セッション, 初期化エラー> {
        eprintln!("initを処理します(operation={操作種別:?})");

        let プロファイル = self.設定の置き場所.論理プロファイルを解決する().map_err(設定エラーへ変換する)?;

        let 保管庫 = 起動確認を済ませた保管庫を組み立てる(&プロファイル)?;

        let 一時ディレクトリ = ダウンロードの一時ファイル置き場を決める(self.設定の置き場所.探索起点())?;
        一時保存先を作成する(&一時ディレクトリ)?;

        let サービス = 資産転送サービス::生成する(保管庫, 一時ディレクトリ);
        Ok(転送セッション::生成する(サービス))
    }
}

/// ダウンロードの一時ファイルは`complete`の応答でGit LFSへ所有権が移り、Git LFSが
/// `rename`でobjectの置き場所へ移す。ボリュームをまたぐ`rename`は失敗するため、置き場所は
/// PC設定ではなくリポジトリ側から決める（`git_lfs_storage_directory.rs`の注意書き）。
fn ダウンロードの一時ファイル置き場を決める(起点ディレクトリ: &std::path::Path) -> Result<一時ディレクトリ, 初期化エラー> {
    let 保管ディレクトリ = GitLFS保管ディレクトリ::作業ディレクトリから問い合わせる(起点ディレクトリ).map_err(|エラー| {
        初期化エラー::保管失敗(保管エラー::ローカル入出力 {
            説明: format!("ダウンロードの一時ファイル置き場を決められませんでした: {エラー}"),
        })
    })?;
    Ok(保管ディレクトリ.ダウンロード一時ディレクトリ())
}
