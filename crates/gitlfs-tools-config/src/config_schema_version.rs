//! 設定ファイルの`schema_version`を表す値型。

use crate::config_error::設定エラー;

const 受理できる版: u64 = 1;

/// プロジェクト設定・PC設定の共通TOMLキー`schema_version`。v1では1だけを受理する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct 設定スキーマ版(u64);

impl 設定スキーマ版 {
    /// 受理できる版と一致しない場合、受信した版と受理できる版の両方を含む
    /// `設定エラー::未対応スキーマ版`を返す。
    pub fn 生成する(受信した版: u64) -> Result<Self, 設定エラー> {
        if 受信した版 == 受理できる版 {
            Ok(Self(受信した版))
        } else {
            Err(設定エラー::未対応スキーマ版 {
                受信した版,
                受理できる版,
            })
        }
    }

    pub fn 値(&self) -> u64 {
        self.0
    }

    /// このagentが現在受理する最新のスキーマ版。雛形生成コマンドが書き込む
    /// `schema_version`の値をこの型経由で取得し、受理側の定数と重複させない。
    pub fn 最新() -> Self {
        Self(受理できる版)
    }
}
