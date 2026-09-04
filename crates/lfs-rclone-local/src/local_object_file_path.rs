//! 保管先の1オブジェクトを、ローカルファイルシステムで開くファイルパスとして表す値型。
//!
//! `保管先オブジェクトパス`（domain層）が持つ`/`区切りの綴りをそのまま`PathBuf`へ渡す。
//! 区切り文字の置換を行わないのは、Windowsのファイル操作APIが`/`を区切りとして受け付け、
//! Unix系では`/`が本来の区切りであるためである。パスの綴りの正本を`保管先基底パス`の
//! 1箇所へ保ち、このクレートで組み立て直さない。

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use lfs_rclone_domain::{保管エラー, 保管先オブジェクトパス, 期待バイト数};

/// 保管先の1オブジェクトのローカルファイルパス。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct 保管先オブジェクトのローカルファイルパス(PathBuf);

impl 保管先オブジェクトのローカルファイルパス {
    pub fn 保管先オブジェクトパスから生成する(パス: &保管先オブジェクトパス) -> Self {
        Self(PathBuf::from(パス.文字列表現()))
    }

    /// ファイルシステムAPIへ渡すための参照（境界1箇所）。
    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// このパスへ書き込めるよう、中間ディレクトリを必要に応じて作る。
    pub fn 中間ディレクトリを用意する(&self) -> Result<(), 保管エラー> {
        let Some(親ディレクトリ) = self.0.parent() else { return Ok(()) };
        std::fs::create_dir_all(親ディレクトリ).map_err(|エラー| 保管エラー::ローカル入出力 {
            説明: format!("保管先の中間ディレクトリを作成できませんでした: {エラー}"),
        })
    }

    /// 実体があればそのバイト数を、無ければ`None`を返す。
    pub fn 実バイト数を調べる(&self) -> Result<Option<期待バイト数>, 保管エラー> {
        match std::fs::metadata(&self.0) {
            Ok(メタデータ) if メタデータ.is_file() => Ok(Some(期待バイト数::生成する(メタデータ.len()))),
            Ok(_) => Err(保管エラー::ローカル入出力 {
                説明: "保管先のオブジェクトパスがファイルではありません".to_owned(),
            }),
            Err(エラー) if エラー.kind() == ErrorKind::NotFound => Ok(None),
            Err(エラー) => Err(保管エラー::ローカル入出力 {
                説明: format!("保管先のオブジェクトの状態を調べられませんでした: {エラー}"),
            }),
        }
    }

    /// 実体を削除する。既に無い場合も成功として扱う。
    pub fn 実体を削除する(&self) -> Result<(), 保管エラー> {
        match std::fs::remove_file(&self.0) {
            Ok(()) => Ok(()),
            Err(エラー) if エラー.kind() == ErrorKind::NotFound => Ok(()),
            Err(エラー) => Err(保管エラー::ローカル入出力 {
                説明: format!("保管先のオブジェクトを削除できませんでした: {エラー}"),
            }),
        }
    }
}
