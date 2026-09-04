//! Git LFS custom transfer agent の実行ファイル入口。コンポジションルート
//! （設定の読み込み・依存の配線・起動）を担う（コード分割規約.md 1節）。
//!
//! 標準出力はGit LFSとの通信専用である。診断情報は標準エラー出力へ書く。ただし
//! 引数付きで起動したサブコマンド（`install`・`init-project`・`doctor`・`help`）は
//! プロトコル通信ではないため、利用者向けの文字列を標準出力へ書いてよい
//! （CLAUDE.md「標準出力の規律」）。引数なしの起動だけがプロトコル通信であり、
//! この分岐（`launch_mode`）を変えるとGit LFSとの通信経路が壊れる。

mod child_process_exit_code;
mod command_error;
mod config_diagnostic;
mod config_error_mapping;
mod deprecated_setting_diagnostic;
mod diagnostic_finding;
mod doctor_command;
mod doctor_scratch_directory;
mod download_temp_directory_diagnostic;
mod git_attributes_diagnostic;
mod git_hook_directory;
mod git_lfs_filter_diagnostic;
mod git_lfs_hook;
mod git_lfs_hook_diagnostic;
mod git_lfs_installation_diagnostic;
mod git_lfs_storage_directory;
mod git_repository;
mod git_transfer_diagnostic;
mod git_transfer_settings;
mod init_project_command;
mod install_command;
mod install_target_path;
mod launch_argument_error;
mod launch_mode;
mod local_storage_write_probe;
mod object_storage_selection;
mod pc_config_location_resolution;
mod project_config_template;
mod rclone_startup_check;
mod storage_assembly;
mod storage_reachability_diagnostic;
mod storage_write_diagnostic;
mod storage_write_probe;
mod subcommand;
mod temp_directory_provisioning;
mod timeout_process_runner;
mod transfer_init_boundary;
mod transfer_session;
mod usage_text;
mod work_tree_root;
mod working_directory_resolution;

use std::env;
use std::process::ExitCode;

use lfs_rclone_protocol::プロトコルセッション;

use crate::launch_mode::起動モード;
use crate::pc_config_location_resolution::pc設定の場所を解決する;
use crate::subcommand::サブコマンド;
use crate::transfer_init_boundary::転送セッション初期化境界;
use crate::usage_text::使い方テキスト;
use crate::working_directory_resolution::作業ディレクトリを解決する;

fn main() -> ExitCode {
    let 引数: Vec<String> = env::args().skip(1).collect();
    match launch_mode::起動モード::起動引数から解釈する(&引数) {
        Ok(起動モード::プロトコル通信) => プロトコル通信で起動する(),
        Ok(起動モード::サブコマンド実行(サブコマンド::導入 { 実行ファイルパス })) => install_command::導入を実行する(実行ファイルパス),
        Ok(起動モード::サブコマンド実行(サブコマンド::雛形生成 { プロファイル })) => init_project_command::雛形生成を実行する(プロファイル),
        Ok(起動モード::サブコマンド実行(サブコマンド::検証)) => doctor_command::検証を実行する(),
        Ok(起動モード::サブコマンド実行(サブコマンド::ヘルプ)) => {
            println!("{使い方テキスト}");
            ExitCode::SUCCESS
        }
        Err(エラー) => {
            eprintln!("{エラー}");
            eprintln!();
            eprintln!("{使い方テキスト}");
            ExitCode::FAILURE
        }
    }
}

fn プロトコル通信で起動する() -> ExitCode {
    let 起点ディレクトリ = match 作業ディレクトリを解決する() {
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

    let 境界 = 転送セッション初期化境界::生成する(起点ディレクトリ, pc設定の場所);
    let セッション = プロトコルセッション::生成する(境界);

    if セッション.実行する().正常か() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
