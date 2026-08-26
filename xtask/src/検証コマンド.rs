//! コミット前に完走させる標準の検証列。
//!
//! 順序に意味がある。型が通らない状態で clippy を回しても診断が出揃わないため、
//! cargo check を先に置く。

use std::process::Command;

use crate::コマンド定義::サブコマンド;

pub struct 検証コマンド;

impl サブコマンド for 検証コマンド {
    fn 名前(&self) -> &'static str {
        "verify"
    }

    fn 説明(&self) -> &'static str {
        "cargo check -> clippy -D warnings -> cargo test を順に実行する"
    }

    fn 実行する(&self, _引数: &[String]) -> Result<(), String> {
        let 工程一覧: [(&str, &[&str]); 3] = [
            ("型検査", &["check", "--workspace", "--all-targets"]),
            (
                "lint検査",
                &["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
            ),
            ("テスト", &["test", "--workspace"]),
        ];

        for (工程名, cargo引数) in 工程一覧 {
            eprintln!("== {工程名}: cargo {} ==", cargo引数.join(" "));
            cargoを実行する(cargo引数).map_err(|失敗| format!("{工程名}で失敗した。{失敗}"))?;
        }
        eprintln!("== 検証列を完走した ==");
        Ok(())
    }
}

fn cargoを実行する(引数: &[&str]) -> Result<(), String> {
    let cargo実行ファイル = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let 結果 = Command::new(cargo実行ファイル)
        .args(引数)
        .status()
        .map_err(|失敗| format!("cargoを起動できなかった: {失敗}"))?;

    if 結果.success() {
        Ok(())
    } else {
        Err(match 結果.code() {
            Some(終了コード) => format!("cargoが終了コード{終了コード}で終わった"),
            None => "cargoがシグナルで終了した".to_owned(),
        })
    }
}
