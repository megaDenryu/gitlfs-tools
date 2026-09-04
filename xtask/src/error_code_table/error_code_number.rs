//! `error.code`として外部へ出る数値。実装と文書を突き合わせるときの照合キーである。

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct エラーコード番号(u32);

impl エラーコード番号 {
    pub fn 生成する(数値: u32) -> Self {
        Self(数値)
    }
}

impl std::fmt::Display for エラーコード番号 {
    fn fmt(&self, 出力先: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(出力先, "{}", self.0)
    }
}
