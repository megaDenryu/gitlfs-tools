//! `模擬PC`の続きの`impl`。対象実行ファイル（本エージェント自身）のサブコマンド起動を持つ。
//! git操作（`pc_environment.rs`）とは叩く相手が別（本エージェント自身）という責務のため
//! ファイルを分ける。

use std::path::Path;
use std::process::Command;

use crate::acceptance_check::pc_environment::{模擬PC, 起動して結果を包む};
use crate::acceptance_check::process_output::子プロセス出力;

impl 模擬PC {
    /// 本エージェントのサブコマンド（`install`・`init-project`・`doctor`等）を、この
    /// PCのPC設定ディレクトリを`LFS_RCLONE_PC_CONFIG_DIR`として渡して実行する。
    pub fn エージェントを実行する(&self, 作業ディレクトリ: &Path, 引数: &[&str]) -> Result<子プロセス出力, String> {
        let mut コマンド = Command::new(self.実行ファイル().パス());
        コマンド.current_dir(作業ディレクトリ).env("LFS_RCLONE_PC_CONFIG_DIR", self.pc設定().パス()).args(引数);
        起動して結果を包む(&mut コマンド, "対象実行ファイル", 引数)
    }
}
