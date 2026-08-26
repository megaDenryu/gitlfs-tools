//! 一時ディレクトリが払い出す固有のローカルファイルパスを表す値型。

use std::path::{Path, PathBuf};

/// 一時ディレクトリが払い出す、衝突しない固有のファイルパス。
///
/// 注意: 生成は`一時ディレクトリ`のメソッド経由に限る。任意のパスから組み立てられる
/// 公開コンストラクタを持たない。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct 一時ファイルパス(PathBuf);

impl 一時ファイルパス {
    pub(crate) fn 生成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// ファイルシステムAPI等、外部境界へ渡すための参照。
    pub fn パス(&self) -> &Path {
        &self.0
    }
}
