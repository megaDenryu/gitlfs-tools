//! 子プロセスの終了コードを、利用者が読む文へ埋め込める形へ整える。
//!
//! `ExitStatus::code()`は`Option<i32>`であり、そのまま書式へ埋めると`Some(1)`のような
//! Rustの`Debug`表現が利用者の目に触れる。Unixではシグナルで終了した場合に終了コードが
//! 存在しないため、その場合を日本語の文で表す。

use std::process::ExitStatus;

/// 終了状態から、利用者向けの終了コードの表記を作る。
pub(crate) fn 子プロセスの終了コードを表す文字列を作る(状態: &ExitStatus) -> String {
    match 状態.code() {
        Some(コード) => コード.to_string(),
        None => "なし(シグナルによる終了)".to_owned(),
    }
}
