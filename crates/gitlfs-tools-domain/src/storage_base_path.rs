//! プロファイルごとの保管先の基点を表す値型。

use crate::object_identifier::オブジェクト識別子;
use crate::storage_error::保管エラー;
use crate::storage_object_path::保管先オブジェクトパス;
use crate::unique_identifier::一意な識別子;

/// プロファイルごとの保管先の基点。区切りは常に`/`とする
/// （保管先はrcloneのリモートであり、Windowsのファイルシステムではない）。
///
/// 保管先オブジェクトのパスの綴りをこの型が知る。呼び出し側で`join`を連ねさせない。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct 保管先基底パス(String);

impl 保管先基底パス {
    pub fn 生成する(基点: impl Into<String>) -> Result<Self, 保管エラー> {
        let 末尾の区切りを除いた基点 = 基点.into().trim_end_matches('/').to_owned();
        if 末尾の区切りを除いた基点.is_empty() {
            return Err(保管エラー::設定不備 {
                説明: "保管先基底パスが空です".to_owned(),
            });
        }
        Ok(Self(末尾の区切りを除いた基点))
    }

    /// オブジェクト識別子から最終保存先を導く。
    ///
    /// 形式: `<基点>/lfs/objects/sha256/<先頭2文字>/<次の2文字>/<64文字の識別子>`
    pub fn オブジェクトパスを求める(&self, 識別子: &オブジェクト識別子) -> 保管先オブジェクトパス {
        保管先オブジェクトパス::生成する(format!(
            "{}/lfs/objects/sha256/{}/{}/{}",
            self.0,
            識別子.先頭2文字(),
            識別子.続く2文字(),
            識別子.文字列表現()
        ))
    }

    /// 保管先のオブジェクト置き場を導く。この下に在るファイルが保管済みのオブジェクトである。
    ///
    /// 形式: `<基点>/lfs/objects`
    ///
    /// この置き場は複数のリポジトリで共用する。ここを数えると、他のリポジトリが置いた
    /// オブジェクトも数に入る。
    pub fn オブジェクト置き場のパスを求める(&self) -> 保管先オブジェクトパス {
        保管先オブジェクトパス::生成する(format!("{}/lfs/objects", self.0))
    }

    /// 一時アップロード先を払い出す。呼ぶたびに異なるパスになる。
    ///
    /// 形式: `<基点>/lfs/tmp/<UUID>`
    pub fn 一時アップロード先を払い出す(&self) -> 保管先オブジェクトパス {
        let 識別子 = 一意な識別子::発行する();
        保管先オブジェクトパス::生成する(format!("{}/lfs/tmp/{}", self.0, 識別子.文字列表現()))
    }
}
