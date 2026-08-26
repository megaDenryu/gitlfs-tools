//! Git LFS custom transfer agent の実行ファイル入口。コンポジションルート
//! （設定の読み込み・依存の配線・起動）を担う（コード分割規約.md 1節）。
//!
//! 標準出力は Git LFS との通信専用である。診断情報は標準エラー出力へ書く。

mod config_error_mapping;
mod rclone_init_boundary;
mod rclone_startup_check;
mod rclone_transfer_session;
mod temp_directory_provisioning;

use std::env;
use std::process::ExitCode;

use lfs_rclone_config::PC設定の場所;
use lfs_rclone_protocol::プロトコルセッション;

use crate::rclone_init_boundary::Rclone初期化境界;

fn main() -> ExitCode {
    let 起点ディレクトリ = match env::current_dir() {
        Ok(ディレクトリ) => ディレクトリ,
        Err(エラー) => {
            eprintln!("作業ディレクトリの取得に失敗しました: {エラー}");
            return ExitCode::FAILURE;
        }
    };

    let pc設定の場所 = match pc設定の場所を解決する() {
        Ok(場所) => 場所,
        Err(エラー) => {
            eprintln!("PC設定の場所を解決できませんでした: {エラー}");
            return ExitCode::FAILURE;
        }
    };

    let 境界 = Rclone初期化境界::生成する(起点ディレクトリ, pc設定の場所);
    let セッション = プロトコルセッション::生成する(境界);

    if セッション.実行する().正常か() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

/// PC設定の置き場所を決める。テスト専用の環境変数`LFS_RCLONE_PC_CONFIG_DIR`が設定されて
/// いれば、実ユーザーの設定ディレクトリの代わりにそのディレクトリを使う
/// （結合テストが実設定ディレクトリを読み書きしないための差し替え口。`lfs-rclone-rclone`の
/// `LFS_RCLONE_TEST_EXECUTABLE`と同じ方針）。
fn pc設定の場所を解決する() -> Result<lfs_rclone_config::PC設定の場所, lfs_rclone_config::設定エラー> {
    match env::var("LFS_RCLONE_PC_CONFIG_DIR") {
        Ok(ディレクトリ) => Ok(PC設定の場所::ディレクトリを指定して生成する(ディレクトリ)),
        Err(_) => PC設定の場所::既定の場所を解決する(),
    }
}
