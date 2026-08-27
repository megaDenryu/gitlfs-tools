//! `主鎖状態`の続きの`impl`。項目3: PC Aの作業treeとLFS cacheを使わないPC B相当の
//! cloneで、同じcommitから同じSHA-256とバイト数のファイルを取得できることを確かめる。
//! Issue #2 7.3節の4手順（smudge抑止clone→設定→`git lfs pull`）で取得する。

use std::ffi::OsStr;
use std::path::Path;

use crate::acceptance_check::scenario_state::{主鎖状態, 追跡ファイル名};
use crate::acceptance_check::sha256_digest::ファイル指紋;

impl 主鎖状態 {
    pub fn 項目3_pcbで同一内容を取得する(&mut self, 作業域ルート: &Path) -> Result<String, String> {
        self.pc_b
            .git実行(
                作業域ルート,
                &[("GIT_LFS_SKIP_SMUDGE", OsStr::new("1"))],
                &["clone", &self.origin.to_string_lossy(), &self.pc_b_workdir.to_string_lossy()],
            )?
            .成功を要求する("PC Bのclone(smudge抑止)")?;
        self.pc_b.git実行(&self.pc_b_workdir, &[], &["lfs", "install", "--local"])?.成功を要求する("PC Bのgit lfs install --local")?;
        self.pc_b.エージェントを実行する(&self.pc_b_workdir, &["install"])?.成功を要求する("PC Bのinstall")?;
        self.pc_b.git実行(&self.pc_b_workdir, &[], &["lfs", "pull"])?.成功を要求する("PC Bのgit lfs pull")?;

        let pc_a指紋 = ファイル指紋::計測する(&self.pc_a_workdir.join(追跡ファイル名))?;
        let pc_b指紋 = ファイル指紋::計測する(&self.pc_b_workdir.join(追跡ファイル名))?;
        if pc_a指紋 != pc_b指紋 {
            return Err(format!("PC AとPC Bの指紋が一致しない: PC A={pc_a指紋:?} PC B={pc_b指紋:?}"));
        }
        Ok(format!("SHA-256={}, バイト数={}で一致した", pc_b指紋.sha256十六進, pc_b指紋.バイト数))
    }
}
