//! 保管操作全体で使うエラー分類。Issue #2の10節が正本である。
//!
//! `整合性エラー`（`crate::integrity_error`）は識別子の構文検証等、保管操作がまだ始まって
//! いない解析でも独立して使う型である。保管操作の文脈で扱うときは`From<整合性エラー>`により
//! `保管エラー::整合性`へ昇格する。

use crate::integrity_error::整合性エラー;
use crate::object_identifier::オブジェクト識別子;

/// 保管操作の失敗を分類する。object単位の失敗として`event=complete`の`error`へ変換される
/// ことを想定し、秘密情報を含まない説明文だけを持つ。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum 保管エラー {
    #[error("設定不備: {説明}")]
    設定不備 { 説明: String },

    #[error("認証または接続に失敗しました: {説明}")]
    認証接続 { 説明: String },

    #[error("オブジェクトが存在しません: {識別子}")]
    未存在 { 識別子: オブジェクト識別子 },

    #[error("整合性エラー: {0}")]
    整合性(整合性エラー),

    #[error("ローカル入出力に失敗しました: {説明}")]
    ローカル入出力 { 説明: String },

    #[error("子プロセスの実行に失敗しました: {説明}")]
    子プロセス { 説明: String },
}

impl From<整合性エラー> for 保管エラー {
    fn from(エラー: 整合性エラー) -> Self {
        保管エラー::整合性(エラー)
    }
}
