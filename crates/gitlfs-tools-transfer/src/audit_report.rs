//! 点検1回分の結果を表す値型。

use crate::missing_object::欠落オブジェクト;

/// 点検1回分の結果。点検した件数を必ず持つのは、欠落が0件だったときに「全部を見て0件」
/// なのか「1件も見ていない」のかを、読み手が区別できるようにするためである
/// （グローバルCLAUDE.md「検査器は検査した件数を報告する」）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 点検報告 {
    点検した件数: usize,
    欠落一覧: Vec<欠落オブジェクト>,
}

impl 点検報告 {
    pub(crate) fn 生成する(点検した件数: usize, 欠落一覧: Vec<欠落オブジェクト>) -> Self {
        Self { 点検した件数, 欠落一覧 }
    }

    pub fn 点検した件数(&self) -> usize {
        self.点検した件数
    }

    pub fn 欠落一覧(&self) -> &[欠落オブジェクト] {
        &self.欠落一覧
    }

    pub fn 全て保管先に在るか(&self) -> bool {
        self.欠落一覧.is_empty()
    }
}
