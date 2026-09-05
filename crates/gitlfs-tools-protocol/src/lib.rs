//! Git LFS custom transfer protocolのJSON入出力と日本語ドメインモデルへの変換を行う
//! アダプタ層（アーキテクチャ.md 2節、Issue #7）。
//!
//! `gitlfs-tools-rclone`・`gitlfs-tools-config`・`gitlfs-tools-storage-port`のいずれにも
//! 依存しない（判断2「プロトコルアダプタは保管庫の実装を知らない」）。`init`が必要とする
//! 「設定を読み、プロファイルを解決し、rcloneの起動可否を確かめ、資産転送サービスを
//! 組み立てる」手続きは`転送セッション開始境界`トレイトの背後に置き、実装は`gitlfs-tools-cli`
//! が持つ。
//!
//! ファイル名はモジュール名と一致する英語のsnake_case、中身の型名・関数名は日本語である
//! （コード分割規約.md 1節「命名」）。

mod error_code;
mod exit_status;
mod incoming_event_json;
mod init_error;
mod init_request;
mod outgoing_event_json;
mod presentable_error;
mod protocol_parse_error;
mod protocol_request;
mod protocol_response_writer;
mod protocol_session;
mod protocol_session_transfer;
mod stdout_writer;
mod transfer_operation_kind;
mod transfer_session_boundary;

pub use exit_status::終了状態;
pub use init_error::初期化エラー;
pub use protocol_session::プロトコルセッション;
pub use transfer_operation_kind::転送操作種別;
pub use transfer_session_boundary::{開始済み転送セッション, 転送セッション開始境界};
