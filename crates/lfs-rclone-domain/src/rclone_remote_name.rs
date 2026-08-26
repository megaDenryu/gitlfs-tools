//! 各PCのrclone設定に存在するリモート名を表す値型。

use crate::storage_error::保管エラー;

/// rcloneのリモート名。2台のPCで実際の名前が違ってもよい（論理プロファイル名で対応づく）。
/// 空文字は許容しない。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Rcloneリモート名(String);

impl Rcloneリモート名 {
    pub fn 生成する(名前: impl Into<String>) -> Result<Self, 保管エラー> {
        let 名前 = 名前.into();
        if 名前.trim().is_empty() {
            return Err(保管エラー::設定不備 {
                説明: "rcloneリモート名が空です".to_owned(),
            });
        }
        Ok(Self(名前))
    }

    /// rcloneのCLI引数組み立て等、外部境界へ渡すための文字列表現。
    pub fn 文字列表現(&self) -> &str {
        &self.0
    }
}
