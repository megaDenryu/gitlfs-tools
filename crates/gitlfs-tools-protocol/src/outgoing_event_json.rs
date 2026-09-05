//! Git LFSへ書き出す応答JSONの生表現。フィールド名は仕様どおりの英語綴りを保持する。
//!
//! `bytesSoFar`・`bytesSinceLast`は仕様がキャメルケースで固定するため、Rustの
//! フィールド名はsnake_caseにして`#[serde(rename)]`で書き出し名だけをキャメルケースへ
//! 戻す（`non_snake_case`はフィールド名にも働くため、Rust識別子側は変えられない）。

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct 確認応答JSON {}

#[derive(Debug, Serialize)]
pub(crate) struct エラーJSON {
    pub(crate) code: u32,
    pub(crate) message: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct 初期化失敗JSON {
    pub(crate) error: エラーJSON,
}

#[derive(Debug, Serialize)]
pub(crate) struct 進捗JSON {
    pub(crate) event: &'static str,
    pub(crate) oid: String,
    #[serde(rename = "bytesSoFar")]
    pub(crate) bytes_so_far: u64,
    #[serde(rename = "bytesSinceLast")]
    pub(crate) bytes_since_last: u64,
}

#[derive(Debug, Serialize)]
pub(crate) struct 完了JSON {
    pub(crate) event: &'static str,
    pub(crate) oid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<エラーJSON>,
}
