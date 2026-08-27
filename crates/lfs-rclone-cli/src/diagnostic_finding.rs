//! `doctor`が報告する1件分の診断結果を表す値型。表示行の組み立てまでを純粋な変換として
//! この型が持ち、標準出力への書き出しは呼び出し側（`doctor_command`）が行う
//! （コード分割規約.md「値型」役割: 入出力を書いてはならない）。

use std::fmt::Display;

pub(crate) enum 診断結果 {
    問題なし { 項目: &'static str },
    不足 { 項目: &'static str, 何が: String, どうすれば直るか: String },
}

impl 診断結果 {
    pub(crate) fn 不足から生成する(項目: &'static str, エラー: &impl Display, どうすれば直るか: &str) -> Self {
        Self::不足 { 項目, 何が: エラー.to_string(), どうすれば直るか: どうすれば直るか.to_owned() }
    }

    pub(crate) fn 揃っているか(&self) -> bool {
        matches!(self, Self::問題なし { .. })
    }

    pub(crate) fn 表示行一覧(&self) -> Vec<String> {
        match self {
            Self::問題なし { 項目 } => vec![format!("[OK] {項目}")],
            Self::不足 { 項目, 何が, どうすれば直るか } => {
                vec![format!("[不足] {項目}: {何が}"), format!("       対処: {どうすれば直るか}")]
            }
        }
    }
}
