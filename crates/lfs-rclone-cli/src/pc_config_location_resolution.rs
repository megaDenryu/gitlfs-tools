//! PC設定の置き場所を決める手続き。プロトコル通信の起動経路と`doctor`サブコマンドの
//! 両方が同じ規則を必要とするため、コンポジションルート直下の共有手続きとして独立させる。
//!
//! テスト専用の環境変数`LFS_RCLONE_PC_CONFIG_DIR`が設定されていれば、実ユーザーの設定
//! ディレクトリの代わりにそのディレクトリを使う（結合テストが実設定ディレクトリを
//! 読み書きしないための差し替え口。`lfs-rclone-rclone`の`LFS_RCLONE_TEST_EXECUTABLE`と
//! 同じ方針）。

use std::env;

use lfs_rclone_config::{PC設定の場所, 設定エラー};

const PC設定ディレクトリ環境変数: &str = "LFS_RCLONE_PC_CONFIG_DIR";

pub(crate) fn pc設定の場所を解決する() -> Result<PC設定の場所, 設定エラー> {
    match env::var(PC設定ディレクトリ環境変数) {
        Ok(ディレクトリ) => Ok(PC設定の場所::ディレクトリを指定して生成する(ディレクトリ)),
        Err(_) => PC設定の場所::既定の場所を解決する(),
    }
}
