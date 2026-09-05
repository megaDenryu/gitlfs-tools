//! rclone local backendの保管先ルート（"Google Drive"の代役）を表す値型。オブジェクトの
//! パスの綴りと、rcloneのlocal backend用のドライブ分解をこの型のメソッドへ閉じる
//! （グローバルCLAUDE.md「役割の型は自分の配置を知る」）。オブジェクトツリーの
//! 数え上げ・削除・破損・複製は`object_storage_maintenance.rs`が続きの`impl`を持つ
//! （保存先の配置と、保存先に対する保守操作という別の責務のため分ける）。

use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct オブジェクト保管ルート(PathBuf);

impl オブジェクト保管ルート {
    pub fn 生成する(ディレクトリ: PathBuf) -> Self {
        Self(ディレクトリ)
    }

    pub fn パス(&self) -> &Path {
        &self.0
    }

    /// rcloneのlocal backend用に、絶対パスをドライブ文字と残りパス（`/`区切り）へ分ける
    /// （`gitlfs-tools-rclone`のlocal backend結合テストと同じ手法）。
    pub fn ドライブと残りへ分解する(&self) -> Result<(String, String), String> {
        let 文字列 = self.0.to_str().ok_or("保管先ルートのパスがUTF-8ではありません")?;
        let 正規化 = 文字列.replace('\\', "/");
        let (ドライブ, 残り) = 正規化.split_once(':').ok_or("保管先ルートに絶対パスのドライブ文字がありません")?;
        Ok((ドライブ.to_owned(), 残り.to_owned()))
    }

    /// content-addressedなオブジェクトパス: `<基底>/lfs/objects/sha256/ab/cd/<64文字のoid>`
    /// （アーキテクチャ.md 判断5）。
    pub fn オブジェクトパス(&self, oid: &str) -> PathBuf {
        self.0.join("lfs").join("objects").join("sha256").join(&oid[0..2]).join(&oid[2..4]).join(oid)
    }

    pub(crate) fn オブジェクトツリーの起点(&self) -> PathBuf {
        self.0.join("lfs").join("objects").join("sha256")
    }
}
