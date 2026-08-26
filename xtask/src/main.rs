//! リポジトリ内ツールの唯一の入口。
//!
//! 外部依存を持たない（std のみ）。起動が重くなるとツールとして使われなくなるためである。

mod command_registry;
mod line_count;
mod verify_command;

use std::process::ExitCode;

use command_registry::サブコマンド登録簿;

fn main() -> ExitCode {
    let 登録簿 = サブコマンド登録簿::既定();
    let 引数: Vec<String> = std::env::args().skip(1).collect();

    let Some(コマンド名) = 引数.first() else {
        登録簿.使い方を表示する();
        return ExitCode::SUCCESS;
    };

    match 登録簿.実行する(コマンド名, &引数[1..]) {
        Ok(()) => ExitCode::SUCCESS,
        Err(失敗) => {
            eprintln!("xtask: {失敗}");
            ExitCode::FAILURE
        }
    }
}
