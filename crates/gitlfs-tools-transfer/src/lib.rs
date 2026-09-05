//! Git LFS objectのアップロード・ダウンロードのユースケースを実装するサービス層。
//!
//! 標準入出力のJSON形式もrcloneのコマンドも知らない。`gitlfs-tools-storage-port`の
//! `オブジェクト保管庫`だけへ依存し、識別子・バイト数・検証済みローカルファイル・一時ファイルの
//! 所有権だけを扱う（アーキテクチャ.md 2節、Issue #6）。
//!
//! ファイル名はモジュール名と一致する英語のsnake_case、中身の型名・関数名は日本語である
//! （コード分割規約.md 1節「命名」）。

mod asset_transfer_service;
mod audit_report;
mod audit_target_object;
mod download_completion;
mod download_request;
mod missing_object;
mod object_presence_audit_service;
mod object_state_consistency;
mod temp_file_cleanup;
mod upload_completion;
mod upload_request;

pub use asset_transfer_service::資産転送サービス;
pub use audit_report::点検報告;
pub use audit_target_object::点検対象オブジェクト;
pub use download_completion::ダウンロード完了;
pub use download_request::ダウンロード要求;
pub use missing_object::{欠落オブジェクト, 欠落の事由};
pub use object_presence_audit_service::保管先オブジェクト在否点検サービス;
pub use upload_completion::アップロード完了;
pub use upload_request::アップロード要求;

/// この層の操作が返すエラー型は`保管エラー`（domain層）そのものである。
///
/// `検証前のローカルファイル::検証する`や`オブジェクト保管庫`の各操作がすでに`保管エラー`で
/// 失敗を返しており、上位層（#7のプロトコルアダプタ）がGit LFSの`error.code`へ変換するために
/// 必要な区別（設定不備・認証接続・未存在・整合性・ローカル入出力・子プロセス）はすべて
/// `保管エラー`の変種が持つ。この層（アップロード・ダウンロードのユースケース）固有の
/// 新しい失敗種別は発生しないため、`保管エラー`をそのまま再輸出する。別名は付けない。
/// `gitlfs-tools-protocol`は`gitlfs_tools_domain`の型（`オブジェクト識別子`等）も直接使うため、
/// 同じ型に2つの名前が並ぶと読み手の区別コストが増える。転送層が固有の失敗種別を持った
/// 時点で、別名でなく本物の新しい型として導入する。
pub use gitlfs_tools_domain::保管エラー;
