//! `主鎖状態`の続きの`impl`。項目6: 同じoidを再pushしても重複オブジェクトが増えない
//! ことを確かめる。

use crate::acceptance_check::scenario_state::主鎖状態;
use crate::acceptance_check::sha256_digest::ファイル指紋;

impl 主鎖状態 {
    pub fn 項目6_同一oid再pushで重複しない(&mut self) -> Result<String, String> {
        let 前 = self.storage_a.オブジェクト一覧を数え上げる()?;
        self.pc_a.git実行(&self.pc_a_workdir, &[], &["lfs", "push", "origin", "main"])?.成功を要求する("PC Aのgit lfs push再実行")?;
        let 後 = self.storage_a.オブジェクト一覧を数え上げる()?;

        if 前 != 後 {
            return Err(format!("再push後にオブジェクト一覧が変化した: 前={前:?} 後={後:?}"));
        }

        let 旧オブジェクトパス = self.storage_a.オブジェクトパス(&self.旧oid);
        let 旧指紋いま = ファイル指紋::計測する(&旧オブジェクトパス)?;
        let 旧指紋直後 = self.旧オブジェクト指紋直後.as_ref().ok_or("項目2の指紋が記録されていない")?;
        if &旧指紋いま != 旧指紋直後 {
            return Err("再push後に旧オブジェクトの内容が変化していた".to_owned());
        }

        Ok(format!("再push前後でオブジェクト数は{}件のまま変化しなかった", 後.len()))
    }
}
