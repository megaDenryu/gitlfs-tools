//! Issue #2 14節「v1受入条件」1〜9番を、実git・実git-lfs・実rcloneを子プロセスとして
//! 動かして確かめるコマンド。実行に時間がかかるため`cargo xtask verify`の工程には含めない。

mod agent_binary;
mod check_result;
mod lfs_pointer;
mod object_storage_maintenance;
mod object_storage_root;
mod pc_config_dir;
mod pc_environment;
mod pc_environment_agent_ops;
mod process_output;
mod report;
mod scenario;
mod scenario_chain_item1;
mod scenario_chain_item2;
mod scenario_chain_item3;
mod scenario_chain_item4;
mod scenario_chain_item5;
mod scenario_chain_item6;
mod scenario_chain_runner;
mod scenario_check_7;
mod scenario_check_8;
mod scenario_check_9;
mod scenario_plain_clone_finding;
mod scenario_setup;
mod scenario_state;
mod sha256_digest;
mod test_payload;
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
