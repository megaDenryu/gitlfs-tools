//! Git LFS objectのアップロード・ダウンロードのユースケースを実装するサービス層。
//!
//! 標準入出力のJSON形式もrcloneのコマンドも知らない。`lfs-rclone-storage-port`の
//! `オブジェクト保管庫`だけへ依存し、識別子・バイト数・検証済み転送元・一時ファイルの
//! 所有権だけを扱う（アーキテクチャ.md 2節、Issue #6）。
//!
//! ファイル名はモジュール名と一致する英語のsnake_case、中身の型名・関数名は日本語である
//! （コード分割規約.md 1節「命名」）。

mod asset_transfer_service;
mod download_completion;
mod download_request;
mod object_state_consistency;
mod temp_file_cleanup;
mod upload_completion;
mod upload_request;

pub use asset_transfer_service::資産転送サービス;
pub use download_completion::ダウンロード完了;
pub use download_request::ダウンロード要求;
pub use upload_completion::アップロード完了;
pub use upload_request::アップロード要求;

/// この層の操作が返すエラー型。
///
/// 判断: 保管操作の失敗分類を表す`保管エラー`（domain層）をそのまま`転送エラー`として
/// 再輸出する。理由は、`検証前のローカルファイル::検証する`や`オブジェクト保管庫`の各操作が
/// すでに`保管エラー`で失敗を返しており、上位層（#7のプロトコルアダプタ）がGit LFSの
/// `error.code`へ変換するために必要な区別（設定不備・認証接続・未存在・整合性・
/// ローカル入出力・子プロセス）はすべて`保管エラー`の変種が持つ。この層（アップロード・
/// ダウンロードのユースケース）固有の新しい失敗種別は発生しないため、変換だけを行う
/// 重複した型を重ねない。
pub use lfs_rclone_domain::保管エラー as 転送エラー;
