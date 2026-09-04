//! プロジェクト設定とPC設定を分離して読み込み、論理プロファイル名を実際のPCプロファイルへ
//! 解決する層（アーキテクチャ.md 2節）。
//!
//! - プロジェクト設定（`.large-assets.toml`）はGit作業ツリーがcommitし、schema版と
//!   論理プロファイル名の2項目だけを持つ。rcloneリモート名・Google Driveのパス・
//!   PCの絶対パス・トークン・client secretを書ける仕様にはしない
//!   （`deny_unknown_fields`で未知キーを拒否する）。
//! - PC設定はGit作業ツリーの外、OS標準の設定ディレクトリに置き、論理プロファイル名から
//!   保管先の種類（`storage`）・`base_path`と、種類ごとの設定値
//!   （rclone子プロセス方式なら`rclone_remote`・転送タイムアウト・`rclone_executable`）を
//!   解決する。`storage`を省略した設定は従来どおりrclone子プロセス方式として扱う。
//!   `temp_directory`は使われなくなった項目であり、読み込みは受け付けるが値を使わない
//!   （`使われなくなった設定項目`）。
//!
//! この層は`domain`層の値型を再利用し、rcloneを起動しない。実行ファイルの存在確認も
//! 行わない（判断1「rcloneアダプタは設定ファイルの形式を知らない」）。
//!
//! ファイル名はモジュール名と一致する英語のsnake_case、中身の型名・関数名は日本語である
//! （コード分割規約.md 1節「命名」）。
//!
//! ## 使い方の例
//!
//! ```no_run
//! use std::env;
//!
//! use lfs_rclone_config::{PC設定の場所, プロジェクト設定の場所};
//!
//! let 作業ディレクトリ = env::current_dir()?;
//! let プロジェクト設定 = プロジェクト設定の場所::探索する(&作業ディレクトリ)?.読み込む()?;
//! let pc設定 = PC設定の場所::既定の場所を解決する()?.読み込む()?;
//! let プロファイル = pc設定.プロファイルを解決する(プロジェクト設定.プロファイル())?;
//! let _ = プロファイル.保管先();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod config_error;
mod config_schema_version;
mod deprecated_setting;
mod local_storage_root;
mod pc_config;
mod pc_config_location;
mod pc_config_toml;
mod pc_profile;
mod project_config;
mod project_config_location;
mod project_config_toml;
mod storage_specification;

pub use config_error::設定エラー;
pub use config_schema_version::設定スキーマ版;
pub use deprecated_setting::使われなくなった設定項目;
pub use local_storage_root::ローカルファイルシステム上の保管先ルート;
pub use pc_config::PC設定;
pub use pc_config_location::PC設定の場所;
pub use pc_profile::PCプロファイル;
pub use project_config::プロジェクト設定;
pub use project_config_location::プロジェクト設定の場所;
pub use storage_specification::保管先の指定;
