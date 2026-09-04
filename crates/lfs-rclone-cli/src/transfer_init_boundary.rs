//! `転送セッション開始境界`の実装。設定読み込み・プロファイル解決・保管先の起動確認・
//! 一時保存先の作成・資産転送サービスの組み立てを行う（アーキテクチャ.md 判断2
//! 「プロトコルアダプタは保管庫の実装を知らない」）。
//!
//! どの方式の保管庫を組み立てるか、その方式にどの起動確認が意味を持つかは
//! `storage_assembly`が決める。

use std::path::PathBuf;

use lfs_rclone_config::{PC設定の場所, プロジェクト設定の場所};
use lfs_rclone_protocol::{初期化エラー, 転送セッション開始境界, 転送操作種別};
use lfs_rclone_transfer::資産転送サービス;

use crate::config_error_mapping::設定エラーへ変換する;
use crate::storage_assembly::起動確認を済ませた保管庫を組み立てる;
use crate::temp_directory_provisioning::一時保存先を作成する;
use crate::transfer_session::転送セッション;

/// `init`要求の起点。プロジェクト設定の探索起点ディレクトリとPC設定の置き場所を保持する。
pub struct 転送セッション初期化境界 {
    起点ディレクトリ: PathBuf,
    pc設定の場所: PC設定の場所,
}

impl 転送セッション初期化境界 {
    pub fn 生成する(起点ディレクトリ: impl Into<PathBuf>, pc設定の場所: PC設定の場所) -> Self {
        Self { 起点ディレクトリ: 起点ディレクトリ.into(), pc設定の場所 }
    }
}

impl 転送セッション開始境界 for 転送セッション初期化境界 {
    type 開始済み転送セッション = 転送セッション;

    fn 開始する(&self, 操作種別: 転送操作種別) -> Result<Self::開始済み転送セッション, 初期化エラー> {
        eprintln!("initを処理します(operation={操作種別:?})");

        let プロジェクト設定 = プロジェクト設定の場所::探索する(&self.起点ディレクトリ)
            .and_then(|場所| 場所.読み込む())
            .map_err(設定エラーへ変換する)?;
        let pc設定 = self.pc設定の場所.読み込む().map_err(設定エラーへ変換する)?;
        let プロファイル = pc設定.プロファイルを解決する(プロジェクト設定.プロファイル()).map_err(設定エラーへ変換する)?;

        let 保管庫 = 起動確認を済ませた保管庫を組み立てる(プロファイル)?;

        let 一時ディレクトリ = プロファイル.一時ディレクトリ().clone();
        一時保存先を作成する(&一時ディレクトリ)?;

        let サービス = 資産転送サービス::生成する(保管庫, 一時ディレクトリ);
        Ok(転送セッション::生成する(サービス))
    }
}
