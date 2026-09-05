//! `git lfs ls-files --json`の出力の形をそのまま写した型（コード分割規約.md「外部表現」）。
//!
//! キー名はGit LFSが決めるため英語のまま保持する。出力には`checkout`・`downloaded`・
//! `oid_type`・`version`も含まれるが、点検に使わないため受け取らない（serdeは未知のキーを
//! 読み飛ばす）。ファイルが1件も無いとき、`files`は空配列ではなく`null`になる
//! （git-lfs 3.5.1で実測）。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct LsFiles出力 {
    pub(crate) files: Option<Vec<LsFiles要素>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LsFiles要素 {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) oid: String,
}
