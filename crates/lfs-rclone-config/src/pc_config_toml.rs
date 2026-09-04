//! PC設定ファイルの生のTOML表現。
//!
//! 役割は解析の入口だけであり、公開APIへ露出させない（`PC設定`・`PCプロファイル`が
//! 公開ドメイン型）。`deny_unknown_fields`により設定ファイルのtypoを早期に検出する。

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PC設定TOML表現 {
    pub(crate) schema_version: u64,
    pub(crate) profiles: HashMap<String, PCプロファイルTOML表現>,
}

/// 保管先の種類（`storage`）によって必要なキーが変わるため、種類ごとに必要・不要が分かれる
/// キーは`Option`で受け、要求の検査は`保管先の指定`が行う（`storage_specification.rs`）。
/// serdeの必須キー検査に任せると、種類に応じた案内文を出せない。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PCプロファイルTOML表現 {
    #[serde(default)]
    pub(crate) storage: Option<String>,
    pub(crate) base_path: String,
    pub(crate) temp_directory: String,
    #[serde(default)]
    pub(crate) rclone_remote: Option<String>,
    #[serde(default)]
    pub(crate) transfer_timeout_seconds: Option<u64>,
    #[serde(default)]
    pub(crate) rclone_executable: Option<String>,
}
