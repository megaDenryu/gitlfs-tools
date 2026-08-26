//! `crates/`と`xtask/`配下のRustファイルをコード行数100行原則で検査するコマンド。
//!
//! 数え方・台帳の扱いはグローバルCLAUDE.md「1ファイル100行の原則と分割の質」に従う。
//! 台帳: `_doc/設計/行数の例外台帳.md`。

mod code_line_count;
mod code_line_measurement;
mod exception_ledger;
mod line_count_limit;
mod relative_rust_file_path;
mod repository_scan;
mod report;
mod rust_source;
mod violation;

use crate::command_registry::サブコマンド;
use exception_ledger::台帳;
use repository_scan::リポジトリルート;

pub struct 行数検査コマンド;

impl サブコマンド for 行数検査コマンド {
    fn 名前(&self) -> &'static str {
        "check-line-count"
    }

    fn 説明(&self) -> &'static str {
        "crates/とxtask/配下の.rsファイルを100行原則と例外台帳で検査する"
    }

    fn 実行する(&self, _引数: &[String]) -> Result<(), String> {
        let ルート = リポジトリルート::カレントディレクトリから生成する()?;
        let 実測一覧 = ルート.コード行数を計測する()?;
        let 台帳 = 台帳::ファイルから読み込む(&ルート.台帳のパス())?;
        let 違反一覧 = 台帳.違反を判定する(&実測一覧);

        report::結果を報告する(&実測一覧, &違反一覧)
    }
}
