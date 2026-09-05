//! rcloneを子プロセスとして起動する`オブジェクト保管庫`の実装クレート。
//!
//! `lfs-rclone-domain`と`lfs-rclone-storage-port`だけを知る。設定ファイルの形式は知らない
//! （アーキテクチャ.md 判断1）。依存（実行ファイルの指定・リモート名・保管先基底パス・
//! 転送タイムアウト）はすべて`domain`の型として構築時に受け取り保持する。
//! rcloneの標準出力・標準エラー出力はこのクレート内で捕捉し、外へ流さない（判断6）。

mod download_transfer;
mod existence_query;
mod finalize_transfer;
mod object_count_query;
mod rclone_execution_error;
mod rclone_object_storage;
mod rclone_operation;
mod rclone_process_runner;
mod upload_transfer;

pub use rclone_object_storage::Rclone保管庫;
