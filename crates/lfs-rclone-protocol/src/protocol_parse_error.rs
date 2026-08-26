//! stdinから受け取った1行をドメインの`転送プロトコル要求`へ変換する過程の失敗分類。
//!
//! 注意: この失敗はGit LFSプロトコル自体の破損に近いとみなす。`init`前に起きた場合は
//! init失敗応答へ変換できるが、`init`後に起きた場合はどの`oid`の要求か特定できず、
//! 個別の`complete`失敗としては返せない（`protocol_session.rs`側の判断。原文はこの
//! 挙動を規定していない: lfs-custom-transfer-protocol.md 10節）。
//!
//! `oid`さえ特定できれば（値の形式が不正でも）個別の`complete`失敗として返し継続する。
//! `size`・`path`の欠落はここでは検出せず、後続の検証（`整合性エラー`・`保管エラー`）へ
//! 委ねる（`protocol_request.rs`の解析を参照）。

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum プロトコル解析エラー {
    #[error("JSON解析に失敗しました: {説明}")]
    JSON解析失敗 { 説明: String },

    #[error("未知のeventを受け取りました: {値}")]
    未知のevent { 値: String },

    #[error("必須フィールドが欠落または不正です: {説明}")]
    必須フィールド欠落または不正 { 説明: String },
}
