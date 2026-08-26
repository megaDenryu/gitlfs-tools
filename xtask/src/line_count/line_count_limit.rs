//! 行数の上限を表す値型。1ファイル原則(100行)と、台帳が許す統合の上限(150行)の
//! 2つの定数、および台帳の登録行が持つ個別の上限をこの型で表す。

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct 行数上限(usize);

impl 行数上限 {
    pub fn 生成する(値: usize) -> Self {
        Self(値)
    }

    pub const fn 原則() -> Self {
        Self(100)
    }

    pub const fn 台帳の許容上限() -> Self {
        Self(150)
    }

    pub fn 数値(&self) -> usize {
        self.0
    }
}
