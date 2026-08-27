//! xtask が受け付けるサブコマンドの登録簿。
//!
//! 新しい手順を作ったときは、シェルスクリプトを増やさずここへ登録する。引数なしで起動した
//! ときの一覧表示が、後続セッションがツールを発見する唯一の手段である。

use crate::acceptance_check::受入試験コマンド;
use crate::line_count::行数検査コマンド;
use crate::verify_command::検証コマンド;

/// サブコマンド1件が満たす契約。
pub trait サブコマンド {
    fn 名前(&self) -> &'static str;
    fn 説明(&self) -> &'static str;
    fn 実行する(&self, 引数: &[String]) -> Result<(), String>;
}

pub struct サブコマンド登録簿 {
    登録済み: Vec<Box<dyn サブコマンド>>,
}

impl サブコマンド登録簿 {
    pub fn 既定() -> Self {
        Self {
            登録済み: vec![Box::new(検証コマンド), Box::new(行数検査コマンド), Box::new(受入試験コマンド)],
        }
    }

    pub fn 使い方を表示する(&self) {
        println!("使い方: cargo xtask <コマンド> [引数...]");
        println!();
        println!("コマンド:");
        for コマンド in &self.登録済み {
            println!("  {:<18} {}", コマンド.名前(), コマンド.説明());
        }
    }

    pub fn 実行する(&self, 名前: &str, 引数: &[String]) -> Result<(), String> {
        let Some(コマンド) = self.登録済み.iter().find(|候補| 候補.名前() == 名前) else {
            self.使い方を表示する();
            return Err(format!("未登録のコマンド: {名前}"));
        };
        コマンド.実行する(引数)
    }
}
