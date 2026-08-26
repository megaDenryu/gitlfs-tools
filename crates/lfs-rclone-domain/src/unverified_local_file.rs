//! 検証前のローカルファイルを表す値型。ファイル入出力を行う唯一の箇所である。

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::expected_byte_count::期待バイト数;
use crate::object_identifier::オブジェクト識別子;
use crate::storage_error::保管エラー;
use crate::verified_source::検証済み転送元;

const 読み取り単位のバイト数: usize = 64 * 1024;

/// 検証前のローカルファイル。バイト数とSHA-256を実測してから`検証済み転送元`へ遷移する。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct 検証前のローカルファイル(PathBuf);

/// `検証前のローカルファイル`が実測した結果。期待値との突き合わせはまだ済んでいない。
pub(crate) struct ローカルファイル計測結果 {
    pub(crate) バイト数: 期待バイト数,
    pub(crate) 内容ハッシュ: オブジェクト識別子,
}

impl 検証前のローカルファイル {
    pub fn 生成する(パス: impl Into<PathBuf>) -> Self {
        Self(パス.into())
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// 自身が指すファイルを読み、バイト数とSHA-256を与えられた識別子・期待バイト数と
    /// 照合する。一致したときだけ`検証済み転送元`を生成する。
    pub fn 検証する(self, 識別子: &オブジェクト識別子, 期待バイト数: 期待バイト数) -> Result<検証済み転送元, 保管エラー> {
        let 計測結果 = self.計測する()?;
        検証済み転送元::生成する(識別子, 期待バイト数, self, 計測結果)
    }

    /// ファイルを読んでバイト数とSHA-256を実測する。期待値との突き合わせは行わない。
    pub(crate) fn 計測する(&self) -> Result<ローカルファイル計測結果, 保管エラー> {
        let 読み取り失敗を変換する = |エラー: std::io::Error| 保管エラー::ローカル入出力 {
            説明: format!("{}の読み取りに失敗しました: {エラー}", self.0.display()),
        };

        let mut ファイル = File::open(&self.0).map_err(読み取り失敗を変換する)?;
        let mut ハッシュ器 = Sha256::new();
        let mut 読み取り済みバイト数: u64 = 0;
        let mut 読み取りバッファ = [0u8; 読み取り単位のバイト数];

        loop {
            let 読み取ったバイト数 = ファイル.read(&mut 読み取りバッファ).map_err(読み取り失敗を変換する)?;
            if 読み取ったバイト数 == 0 {
                break;
            }

            ハッシュ器.update(&読み取りバッファ[..読み取ったバイト数]);

            let 読み取ったバイト数 = u64::try_from(読み取ったバイト数).map_err(|エラー| 保管エラー::ローカル入出力 {
                説明: format!("読み取りバイト数の変換に失敗しました: {エラー}"),
            })?;
            読み取り済みバイト数 += 読み取ったバイト数;
        }

        let ダイジェスト: [u8; 32] = ハッシュ器.finalize().into();

        Ok(ローカルファイル計測結果 {
            バイト数: 期待バイト数::生成する(読み取り済みバイト数),
            内容ハッシュ: オブジェクト識別子::ダイジェストから生成する(ダイジェスト),
        })
    }
}
