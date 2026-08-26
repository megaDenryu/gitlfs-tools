//! リポジトリルートからの相対パスで表した、走査対象の`.rs`ファイルの場所。
//!
//! 台帳の登録行と実測結果を同じ表記(`/`区切り)で突き合わせるための値型である。

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct 相対rustファイルパス(String);

impl 相対rustファイルパス {
    pub(crate) fn 生成する(正規化済み文字列: String) -> Self {
        Self(正規化済み文字列)
    }

    pub fn 文字列表現(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for 相対rustファイルパス {
    fn fmt(&self, フォーマッタ: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(フォーマッタ, "{}", self.0)
    }
}
