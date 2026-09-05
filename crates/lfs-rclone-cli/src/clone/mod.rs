//! `clone`サブコマンドの実装一式。初回cloneの4手順を1コマンドへまとめる（Issue #11）。
//! 選んだ形と理由は`command.rs`の冒頭にある。
//!
//! 1つの概念のファイルが3本以上になったためディレクトリへ昇格させ、ここを集約点にする
//! （コード分割規約.md 3節の昇格経路）。

pub(crate) mod command;
pub(crate) mod error;
pub(crate) mod git_clone_process;
pub(crate) mod git_lfs_repository_setup;
pub(crate) mod source_url;
pub(crate) mod target_directory;
