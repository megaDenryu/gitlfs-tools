//! `主鎖状態`の続きの`impl`。項目5: PC A相当の環境で新旧commitを切り替え、それぞれの
//! 内容を復元できることを確かめる。

use crate::acceptance_check::scenario_state::{主鎖状態, 追跡ファイル名};
use crate::acceptance_check::test_payload::決定的な内容を作る;

impl 主鎖状態 {
    pub fn 項目5_新旧commitを復元する(&mut self) -> Result<String, String> {
        self.pc_a.git実行(&self.pc_a_workdir, &[], &["fetch", "origin"])?.成功を要求する("PC Aのfetch")?;
        self.pc_a.git実行(&self.pc_a_workdir, &[], &["merge", "--ff-only", "origin/main"])?.成功を要求する("PC Aのfast-forward")?;

        let 新commit出力 = self.pc_a.git実行(&self.pc_a_workdir, &[], &["rev-parse", "HEAD"])?.成功を要求する("新HEADの取得")?;
        self.新commit = 新commit出力.標準出力.trim().to_owned();

        let 新内容期待 = 決定的な内容を作る(450_000, 2);
        let 新実際 = std::fs::read(self.pc_a_workdir.join(追跡ファイル名)).map_err(|失敗| format!("新commit checkout後の読み取りに失敗: {失敗}"))?;
        if 新実際 != 新内容期待 {
            return Err("新commitへのfast-forward後の内容が期待と一致しない".to_owned());
        }

        self.pc_a.git実行(&self.pc_a_workdir, &[], &["checkout", &self.旧commit])?.成功を要求する("PC Aの旧commitへのcheckout")?;
        let 旧内容期待 = 決定的な内容を作る(300_000, 1);
        let 旧実際 = std::fs::read(self.pc_a_workdir.join(追跡ファイル名)).map_err(|失敗| format!("旧commit checkout後の読み取りに失敗: {失敗}"))?;
        if 旧実際 != 旧内容期待 {
            return Err("旧commitへのcheckout後の内容が期待と一致しない".to_owned());
        }

        self.pc_a.git実行(&self.pc_a_workdir, &[], &["checkout", "main"])?.成功を要求する("PC Aのmainへの復帰")?;

        Ok(format!("新commit({})と旧commit({})の双方を復元できた", &self.新commit[..12], &self.旧commit[..12]))
    }
}
