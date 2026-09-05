//! 保管先に置かれているオブジェクトの総数を表す値型。

/// 保管先のオブジェクト置き場（`<基底パス>/lfs/objects`）に在るオブジェクトの総数。
///
/// 保管先は複数のリポジトリで共用するため、この数には他のリポジトリが置いたオブジェクトも
/// 含まれる。したがって、この数を特定のリポジトリの期待件数と突き合わせてはならない。
/// 「根ごと消えた」ことに人が気づくための表示にだけ使う。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 保管オブジェクト総数(u64);

impl 保管オブジェクト総数 {
    pub fn 生成する(件数: u64) -> Self {
        Self(件数)
    }

    /// 表示や比較のための数値。
    pub fn 件数(&self) -> u64 {
        self.0
    }
}

/// 利用者が読む表示へ埋め込むための数値。単位は埋め込む側の文が担う。
impl std::fmt::Display for 保管オブジェクト総数 {
    fn fmt(&self, 出力先: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(出力先, "{}", self.0)
    }
}
