//! コード行数(空行とコメントだけの行を除いた行数)を表す値型。
//!
//! 数え方はグローバルCLAUDE.md「1ファイル100行の原則と分割の質」に従う。

use crate::line_count::line_count_limit::行数上限;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct コード行数(usize);

impl コード行数 {
    pub(crate) fn 生成する(値: usize) -> Self {
        Self(値)
    }

    pub fn 数値(&self) -> usize {
        self.0
    }

    pub fn 上限を超えているか(&self, 上限: 行数上限) -> bool {
        self.0 > 上限.数値()
    }

    pub fn 上限との差(&self, 上限: 行数上限) -> usize {
        self.0.abs_diff(上限.数値())
    }
}
