//! `開始済み転送セッション`の実装。`資産転送サービス<Rclone保管庫>`をそのまま委譲する。

use lfs_rclone_domain::保管エラー;
use lfs_rclone_protocol::開始済み転送セッション;
use lfs_rclone_rclone::Rclone保管庫;
use lfs_rclone_transfer::{アップロード完了, アップロード要求, ダウンロード完了, ダウンロード要求, 資産転送サービス};

pub struct Rclone転送セッション(資産転送サービス<Rclone保管庫>);

impl Rclone転送セッション {
    pub(crate) fn 生成する(サービス: 資産転送サービス<Rclone保管庫>) -> Self {
        Self(サービス)
    }
}

impl 開始済み転送セッション for Rclone転送セッション {
    fn アップロードする(&self, 要求: アップロード要求) -> Result<アップロード完了, 保管エラー> {
        self.0.アップロードする(要求)
    }

    fn ダウンロードする(&self, 要求: ダウンロード要求) -> Result<ダウンロード完了, 保管エラー> {
        self.0.ダウンロードする(要求)
    }
}
