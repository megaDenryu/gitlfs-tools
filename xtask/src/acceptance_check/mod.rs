//! Issue #2 14節「v1受入条件」1〜9番を、実git・実git-lfs・実rcloneを子プロセスとして
//! 動かして確かめるコマンド。実行に時間がかかるため`cargo xtask verify`の工程には含めない。

mod agent_binary;
mod check_result;
mod checkout_restores_both_commits;
mod clone_matches_checksum;
mod download_fails_on_missing_or_corrupt;
mod lfs_pointer;
mod object_storage_maintenance;
mod object_storage_root;
mod pc_config_dir;
mod pc_environment;
mod pc_environment_agent_ops;
mod pointer_only_commit;
mod process_output;
mod push_creates_single_object;
mod report;
mod repush_avoids_duplicate;
mod restore_after_backend_replacement;
mod scenario;
mod scenario_chain_runner;
mod scenario_plain_clone_finding;
mod scenario_setup;
mod scenario_state;
mod sha256_digest;
mod test_payload;
mod tracked_files_have_no_secrets;
mod update_adds_new_object;
mod workspace;

use crate::command_registry::サブコマンド;

pub struct 受入試験コマンド;

impl サブコマンド for 受入試験コマンド {
    fn 名前(&self) -> &'static str {
        "check-v1-acceptance"
    }

    fn 説明(&self) -> &'static str {
        "Issue #2 14節のv1受入条件1〜9番を実git/git-lfs/rcloneで実証する(verifyには含めない)"
    }

    fn 実行する(&self, _引数: &[String]) -> Result<(), String> {
        scenario::実行する()
    }
}
