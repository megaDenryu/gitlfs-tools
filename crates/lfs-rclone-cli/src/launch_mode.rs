//! 起動引数から、プロトコル通信とサブコマンド実行のどちらへ進むかを判定する。
//! Git LFSは引数なしでagentを起動するため、この判定を変えるとGit LFSとの通信経路が
//! 壊れる。引数が1つでもあればサブコマンドとして解釈し、プロトコル通信へは進まない
//! （未知の引数を黙ってプロトコル通信として扱うと、利用者が誤字に気づけない）。

use crate::launch_argument_error::起動引数エラー;
use crate::subcommand::サブコマンド;

pub(crate) enum 起動モード {
    プロトコル通信,
    サブコマンド実行(サブコマンド),
}

impl 起動モード {
    pub(crate) fn 起動引数から解釈する(引数: &[String]) -> Result<Self, 起動引数エラー> {
        match 引数.split_first() {
            None => Ok(Self::プロトコル通信),
            Some((先頭, 残り)) => サブコマンド::解釈する(先頭, 残り).map(Self::サブコマンド実行),
        }
    }
}
