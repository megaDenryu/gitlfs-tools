//! `主鎖状態`の続きの`impl`。項目4: PC Bで内容を変更してpushすると新しいoidの
//! オブジェクトが追加され、古いオブジェクトは上書きされないことを確かめる。

use crate::acceptance_check::lfs_pointer;
use crate::acceptance_check::scenario_state::{主鎖状態, 追跡ファイル名};
use crate::acceptance_check::sha256_digest::ファイル指紋;
use crate::acceptance_check::test_payload::決定的な内容を作る;

impl 主鎖状態 {
    pub fn 項目4_pcbの更新で新オブジェクトが追加される(&mut self) -> Result<String, String> {
        let 内容 = 決定的な内容を作る(450_000, 2);
        let 対象ファイル = self.pc_b_workdir.join(追跡ファイル名);
        std::fs::write(&対象ファイル, &内容).map_err(|失敗| format!("{}を書き込めなかった: {失敗}", 対象ファイル.display()))?;

        self.pc_b.git実行(&self.pc_b_workdir, &[], &["add", "-A"])?.成功を要求する("PC Bのgit add")?;
        self.pc_b.コミットする(&self.pc_b_workdir, "PC Bで内容を更新")?.成功を要求する("PC Bのcommit")?;
        self.pc_b.git実行(&self.pc_b_workdir, &[], &["push", "origin", "main"])?.成功を要求する("PC Bからのpush")?;

        let pointer出力 =
            self.pc_b.git実行(&self.pc_b_workdir, &[], &["show", &format!("HEAD:{追跡ファイル名}")])?.成功を要求する("commit blobの取得")?;
        let (新oid, _size) = lfs_pointer::oidとサイズを取り出す(&pointer出力.標準出力)?;
        if 新oid == self.旧oid {
            return Err("PC Bの変更後もoidが変化しなかった".to_owned());
        }
        self.新oid = 新oid.clone();

        let 新オブジェクトパス = self.storage_a.オブジェクトパス(&self.新oid);
        if !新オブジェクトパス.is_file() {
            return Err(format!("新oidのオブジェクトが作られなかった: {}", 新オブジェクトパス.display()));
        }

        let 旧オブジェクトパス = self.storage_a.オブジェクトパス(&self.旧oid);
        let 旧指紋いま = ファイル指紋::計測する(&旧オブジェクトパス)?;
        let 旧指紋直後 = self.旧オブジェクト指紋直後.as_ref().ok_or("項目2の指紋が記録されていない")?;
        if &旧指紋いま != 旧指紋直後 {
            return Err("古いoidのオブジェクトが変化していた(上書きされた疑い)".to_owned());
        }

        Ok(format!("新oid({新oid})のオブジェクトが追加され、旧oid({})のオブジェクトは変化しなかった", self.旧oid))
    }
}
