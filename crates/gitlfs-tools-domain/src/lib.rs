//! Git LFS objectの意味を表す日本語ドメイン型を持つ最下層クレート。
//!
//! 他のどの層も知らない（アーキテクチャ.md 2節）。OID・バイト数・パス・プロファイル名を
//! 裸の`String`・`u64`・`PathBuf`として横流ししないための値型と、保管操作のエラー分類を
//! 提供する。
//!
//! ファイル名はモジュール名と一致する英語のsnake_case、中身の型名・関数名は日本語である
//! （コード分割規約.md 1節「命名」）。

mod expected_byte_count;
mod integrity_error;
mod local_file_measurement;
mod object_identifier;
mod object_state;
mod profile_name;
mod rclone_executable_location;
mod rclone_remote_name;
mod storage_base_path;
mod storage_error;
mod storage_object_path;
mod temp_directory;
mod temp_file_path;
mod transfer_timeout;
mod unique_identifier;
mod verified_local_file;

pub use expected_byte_count::期待バイト数;
pub use integrity_error::整合性エラー;
pub use local_file_measurement::検証前のローカルファイル;
pub use object_identifier::オブジェクト識別子;
pub use object_state::オブジェクト状態;
pub use profile_name::プロファイル名;
pub use rclone_executable_location::Rclone実行ファイルの場所;
pub use rclone_remote_name::Rcloneリモート名;
pub use storage_base_path::保管先基底パス;
pub use storage_error::保管エラー;
pub use storage_object_path::保管先オブジェクトパス;
pub use temp_directory::一時ディレクトリ;
pub use temp_file_path::一時ファイルパス;
pub use transfer_timeout::転送タイムアウト;
pub use verified_local_file::検証済みローカルファイル;
