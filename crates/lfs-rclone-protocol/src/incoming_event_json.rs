//! Git LFS custom transfer protocolから受け取る1行分のJSONの生表現。
//!
//! フィールド名は仕様どおりの英語綴りを保持する（CLAUDE.md「命名」）。イベントの種類に
//! よって使うフィールドが異なるため、全フィールドを`Option`にした1つの表現で受ける。
//! `remote`・`concurrent`・`concurrenttransfers`・`action`等、この型が使わない
//! フィールドは`deny_unknown_fields`を付けず読み捨てる
//! （参照: lfs-custom-transfer-protocol.md 1〜3節）。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct 受信イベントJSON {
    pub(crate) event: String,
    pub(crate) operation: Option<String>,
    pub(crate) oid: Option<String>,
    pub(crate) size: Option<u64>,
    pub(crate) path: Option<String>,
}
