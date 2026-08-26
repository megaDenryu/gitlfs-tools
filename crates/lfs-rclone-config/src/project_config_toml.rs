//! `.large-assets.toml`の生のTOML表現。
//!
//! 役割は解析の入口だけであり、公開APIへ露出させない（`プロジェクト設定`が公開ドメイン型）。
//! `deny_unknown_fields`により、rcloneリモート名・Google Driveのパス・PCの絶対パス・
//! トークン・client secret等の未知キーを明示的に拒否する（Issue #4 完了条件）。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct プロジェクト設定TOML表現 {
    pub(crate) schema_version: u64,
    pub(crate) profile: String,
}
