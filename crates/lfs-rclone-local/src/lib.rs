//! マウント済みのローカルファイルシステム上のディレクトリを保管先とする
//! `オブジェクト保管庫`の実装クレート。
//!
//! `lfs-rclone-domain`と`lfs-rclone-storage-port`だけを知る。設定ファイルの形式は知らない
//! （アーキテクチャ.md 判断1と同じ向きにする）。子プロセスを一切起動せず、標準ライブラリの
//! ファイル操作だけで保存・取得を行うため、保管先がマウント済みのクラウドストレージや
//! ネットワークドライブである場合にrcloneの起動費用を払わずに済む。
//!
//! 保管先のパスの綴りは`lfs-rclone-domain`の`保管先基底パス`が持つものをそのまま使い、
//! このクレートでは組み立て直さない（同じオブジェクトがrcloneの実装と同じ位置へ落ちる）。
//!
//! ファイル名はモジュール名と一致する英語のsnake_case、中身の型名・関数名は日本語である
//! （コード分割規約.md 1節「命名」）。

mod local_directory_storage;
mod local_object_directory;
mod local_object_file_path;
mod local_storage_root;
mod upload_placement;

pub use local_directory_storage::ローカルディレクトリ保管庫;
pub use local_object_file_path::保管先オブジェクトのローカルファイルパス;
pub use local_storage_root::ローカル保管先ルートディレクトリ;
