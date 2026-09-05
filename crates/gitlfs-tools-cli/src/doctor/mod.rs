//! `doctor`サブコマンドの実装一式。設定・保管先・Gitの各種登録を1項目ずつ診断し、
//! 不足があれば利用者が次に取るべき行動とともに報告する（Issue #8「設定を検証する」節）。
//! 一覧の組み立ては`command.rs`が行い、1項目分の判定は項目ごとの別ファイルが持つ。
//!
//! 1つの概念のファイルが3本以上になったためディレクトリへ昇格させ、ここを集約点にする
//! （コード分割規約.md 3節の昇格経路）。

pub(crate) mod command;
pub(crate) mod config_diagnostic;
pub(crate) mod deprecated_setting_diagnostic;
pub(crate) mod download_temp_directory_diagnostic;
pub(crate) mod finding;
pub(crate) mod git_attributes_diagnostic;
pub(crate) mod git_lfs_filter_diagnostic;
pub(crate) mod git_lfs_hook_diagnostic;
pub(crate) mod git_lfs_installation_diagnostic;
pub(crate) mod git_transfer_diagnostic;
pub(crate) mod local_storage_write_probe;
pub(crate) mod program_version_diagnostic;
pub(crate) mod scratch_directory;
pub(crate) mod storage_reachability_diagnostic;
pub(crate) mod storage_write_diagnostic;
pub(crate) mod storage_write_probe;
pub(crate) mod stored_object_count_diagnostic;
