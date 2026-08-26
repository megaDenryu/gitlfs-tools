//! Git LFSのOID（Object ID。内容のSHA-256値）を表す値型。

use crate::storage_error::{保管エラー, 整合性エラー};

const 識別子の文字数: usize = 64;

/// Git LFSのOID。64文字の小文字16進SHA-256文字列でなければ生成できない。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct オブジェクト識別子(String);

impl オブジェクト識別子 {
    /// Git LFSプロトコルから受け取った文字列を検証して生成する。
    ///
    /// 64文字の小文字16進文字列でなければ`保管エラー::整合性`を返す。
    pub fn 生成する(文字列: &str) -> Result<Self, 保管エラー> {
        let 妥当な形式 = 文字列.len() == 識別子の文字数
            && 文字列
                .bytes()
                .all(|バイト| バイト.is_ascii_digit() || (b'a'..=b'f').contains(&バイト));

        if 妥当な形式 {
            Ok(Self(文字列.to_owned()))
        } else {
            Err(保管エラー::整合性(整合性エラー::不正な識別子形式 {
                入力文字列: 文字列.to_owned(),
            }))
        }
    }

    /// ローカルファイルを実測して得たSHA-256ダイジェストから生成する。
    ///
    /// ダイジェストは計算結果そのものであり形式が保証されているため、
    /// `生成する`の文字列検証は経由しない。
    pub(crate) fn ダイジェストから生成する(ダイジェスト: [u8; 32]) -> Self {
        let 十六進文字列 = ダイジェスト.iter().map(|バイト| format!("{バイト:02x}")).collect();
        Self(十六進文字列)
    }

    /// rclone引数・protocol JSON等、外部境界へ渡すための文字列表現。
    pub fn 文字列表現(&self) -> &str {
        &self.0
    }

    /// 保管先オブジェクトパスの階層分割に使う先頭2文字と次の2文字。
    pub(crate) fn 先頭2文字と次の2文字(&self) -> (&str, &str) {
        (&self.0[0..2], &self.0[2..4])
    }
}
