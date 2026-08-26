//! Rustソースの内容から、コード行数を数える純粋な計算。
//!
//! 注意: 文字列リテラルの中に`//`や`/*`が現れる場合、および1行の中でコードと
//! ブロックコメントの開始が同居する場合の判定は、実装の複雑さに見合わないため
//! 考慮しない。行頭が`//`の行コメントと、行頭が`/*`のブロックコメント開始行だけを
//! コメントとして扱う。

use crate::line_count::code_line_count::コード行数;

pub struct Rustソース(String);

impl Rustソース {
    pub fn 生成する(内容: String) -> Self {
        Self(内容)
    }

    pub fn コード行数を数える(&self) -> コード行数 {
        let mut 行数 = 0usize;
        let mut ブロックコメント内 = false;

        for 生の行 in self.0.lines() {
            let 行 = 生の行.trim();

            if ブロックコメント内 {
                if 行.contains("*/") {
                    ブロックコメント内 = false;
                }
                continue;
            }

            if 行.is_empty() || 行.starts_with("//") {
                continue;
            }

            if 行.starts_with("/*") {
                if !行.contains("*/") {
                    ブロックコメント内 = true;
                }
                continue;
            }

            行数 += 1;
        }

        コード行数::生成する(行数)
    }
}
