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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PCプロファイルTOML表現 {
    pub(crate) rclone_remote: String,
    pub(crate) base_path: String,
    pub(crate) temp_directory: String,
    pub(crate) transfer_timeout_seconds: u64,
    #[serde(default)]
    pub(crate) rclone_executable: Option<String>,
}
