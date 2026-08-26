//! 乱数生成（UUID v4）を1箇所へ閉じるための内部値型。
//!
//! 注意: crate内で一時パスを払い出す箇所（`一時ディレクトリ`・`保管先基底パス`）が
//! 各自`uuid::Uuid::new_v4()`を直接呼ぶと、乱数という暗黙のグローバル依存への直叩きが
//! 複数箇所へ散る。この型へ払い出し操作を閉じ、直叩きの箇所を1つに保つ。

/// crate内部の払い出し用途に限る一意な識別子。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct 一意な識別子(uuid::Uuid);

impl 一意な識別子 {
    pub(crate) fn 発行する() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// パスの構成要素として埋め込むための文字列表現。
    pub(crate) fn 文字列表現(&self) -> String {
        self.0.to_string()
    }
}
