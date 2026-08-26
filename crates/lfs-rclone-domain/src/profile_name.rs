//! 論理プロファイル名を表す値型。

use crate::storage_error::保管エラー;

/// プロジェクトが参照する論理プロファイル名。空文字は許容しない。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct プロファイル名(String);

impl プロファイル名 {
    pub fn 生成する(名前: impl Into<String>) -> Result<Self, 保管エラー> {
        let 名前 = 名前.into();
        if 名前.trim().is_empty() {
            return Err(保管エラー::設定不備 {
                説明: "プロファイル名が空です".to_owned(),
            });
        }
        Ok(Self(名前))
    }

    /// 設定ファイルの表示・照合等、外部境界へ渡すための文字列表現。
    pub fn 文字列表現(&self) -> &str {
        &self.0
    }
}
