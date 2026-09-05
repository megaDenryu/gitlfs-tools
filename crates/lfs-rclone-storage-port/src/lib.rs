//! 保管境界クレート。`オブジェクト保管庫`トレイトとその戻り値型を持つ。
//!
//! `lfs-rclone-domain`だけを知る。実装の詳細（rcloneの起動方法等）はここに置かない
//! （アーキテクチャ.md 2節）。

mod object_storage;
mod stored_object_count;
mod upload_result;

pub use object_storage::オブジェクト保管庫;
pub use stored_object_count::保管オブジェクト総数;
pub use upload_result::アップロード結果;
