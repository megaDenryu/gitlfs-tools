//! `主鎖状態`の続きの`impl`。項目1: 新しい隔離Gitリポジトリで試験ファイルをLFS追跡し、
//! commitにはpointerだけが入ることを確かめる。

use std::path::Path;

use crate::acceptance_check::lfs_pointer;
use crate::acceptance_check::scenario_state::{プロファイル名, 主鎖状態, 追跡ファイル名};
use crate::acceptance_check::test_payload::決定的な内容を作る;

impl 主鎖状態 {
    pub fn 項目1_pointerのみコミットする(&mut self, 作業域ルート: &Path) -> Result<String, String> {
        std::fs::create_dir_all(&self.pc_a_workdir).map_err(|失敗| format!("{}を作成できなかった: {失敗}", self.pc_a_workdir.display()))?;
        self.pc_a
            .git実行(作業域ルート, &[], &["-c", "init.defaultBranch=main", "init", &self.pc_a_workdir.to_string_lossy()])?
            .成功を要求する("PC Aのgit init")?;
        self.pc_a.git実行(&self.pc_a_workdir, &[], &["lfs", "install", "--local"])?.成功を要求する("PC Aのgit lfs install --local")?;
        self.pc_a.git実行(&self.pc_a_workdir, &[], &["lfs", "track", "*.bin"])?.成功を要求する("PC Aのgit lfs track")?;

        self.pc_a.エージェントを実行する(&self.pc_a_workdir, &["init-project", "--profile", プロファイル名])?.成功を要求する("PC Aのinit-project")?;
        self.pc_a.エージェントを実行する(&self.pc_a_workdir, &["install"])?.成功を要求する("PC Aのinstall")?;

        let 内容 = 決定的な内容を作る(300_000, 1);
        let 対象ファイル = self.pc_a_workdir.join(追跡ファイル名);
        std::fs::write(&対象ファイル, &内容).map_err(|失敗| format!("{}を書き込めなかった: {失敗}", 対象ファイル.display()))?;

        self.pc_a.git実行(&self.pc_a_workdir, &[], &["add", "-A"])?.成功を要求する("PC Aのgit add")?;
        self.pc_a.コミットする(&self.pc_a_workdir, "初回: 試験ファイルを追加")?.成功を要求する("PC Aの初回commit")?;

        let commit出力 = self.pc_a.git実行(&self.pc_a_workdir, &[], &["rev-parse", "HEAD"])?.成功を要求する("HEADの取得")?;
        self.旧commit = commit出力.標準出力.trim().to_owned();

        let pointer出力 =
            self.pc_a.git実行(&self.pc_a_workdir, &[], &["show", &format!("HEAD:{追跡ファイル名}")])?.成功を要求する("commit blobの取得")?;
        let (oid, size) = lfs_pointer::oidとサイズを取り出す(&pointer出力.標準出力)?;
        if size != u64::try_from(内容.len()).unwrap_or(0) {
            return Err(format!("pointerのsize({size})が実ファイルのバイト数({})と食い違う", 内容.len()));
        }
        self.旧oid = oid.clone();

        Ok(format!(
            "commitのblobはpointer本文({}バイト)であり、working treeの実ファイルは{}バイトだった。oid={oid}",
            pointer出力.標準出力.trim().len(),
            内容.len()
        ))
    }
}
