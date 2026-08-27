//! `主鎖状態`の続きの`impl`。項目2: PC A相当の設定でpushすると、規定の
//! content-addressedなオブジェクトパスへ実体が1個だけ作られることを確かめる。

use std::path::Path;

use crate::acceptance_check::scenario_setup;
use crate::acceptance_check::scenario_state::主鎖状態;
use crate::acceptance_check::sha256_digest::ファイル指紋;

impl 主鎖状態 {
    pub fn 項目2_push後に実体が1個だけ作られる(&mut self, 作業域ルート: &Path) -> Result<String, String> {
        scenario_setup::裸リポジトリを作る(&self.pc_a, 作業域ルート, &self.origin)?;
        self.pc_a
            .git実行(&self.pc_a_workdir, &[], &["remote", "add", "origin", &self.origin.to_string_lossy()])?
            .成功を要求する("originの登録")?;
        self.pc_a.git実行(&self.pc_a_workdir, &[], &["push", "origin", "main"])?.成功を要求する("PC Aからのpush")?;

        let オブジェクトパス = self.storage_a.オブジェクトパス(&self.旧oid);
        if !オブジェクトパス.is_file() {
            return Err(format!("push後に{}が作られなかった", オブジェクトパス.display()));
        }
        let 一覧 = self.storage_a.オブジェクト一覧を数え上げる()?;
        if 一覧.len() != 1 {
            return Err(format!("push後の保管先オブジェクト数が1ではなく{}件だった: {一覧:?}", 一覧.len()));
        }
        self.旧オブジェクト指紋直後 = Some(ファイル指紋::計測する(&オブジェクトパス)?);

        Ok(format!("保管先に実体が1個だけ作られた: {}", オブジェクトパス.display()))
    }
}
