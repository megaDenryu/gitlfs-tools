//! `init`要求の`operation`フィールドが示す転送方向を表す値型。

use crate::protocol_parse_error::プロトコル解析エラー;

/// `operation`が示す転送方向。standalone agentが対応するのは`upload`と`download`だけである
/// （参照: lfs-custom-transfer-protocol.md 1節）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 転送操作種別 {
    アップロード,
    ダウンロード,
}

impl 転送操作種別 {
    /// `operation`フィールドの生文字列から解決する。`upload`/`download`以外は
    /// `プロトコル解析エラー::必須フィールド欠落または不正`とする。
    pub(crate) fn 文字列から生成する(値: Option<&str>) -> Result<Self, プロトコル解析エラー> {
        match 値 {
            Some("upload") => Ok(Self::アップロード),
            Some("download") => Ok(Self::ダウンロード),
            _ => Err(プロトコル解析エラー::必須フィールド欠落または不正 {
                説明: format!(
                    "initのoperationがupload/downloadのいずれでもありません: {}",
                    値.unwrap_or("(未指定)")
                ),
            }),
        }
    }
}
