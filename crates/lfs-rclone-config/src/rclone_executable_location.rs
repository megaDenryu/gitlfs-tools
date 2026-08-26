//! PC設定`rclone_executable`から解決した、rclone実行ファイルの場所を表すドメインモデル。

use std::path::PathBuf;

/// rclone実行ファイルの場所。`rclone_executable`が省略されたときの「PATHへ委ねる」と、
/// 明示されたときの「指定パスを使う」を、空文字や特定文字列への密輸なしに区別する
/// （グローバルCLAUDE.md「『不在・未設定・使用不可』状態の設計」）。
///
/// この層はrcloneを起動しない。実行ファイルが実在するかどうかの確認は
/// 起動側（`lfs-rclone-rclone`）の責務であり、ここでは行わない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rclone実行ファイルの場所 {
    /// PC設定`rclone_executable`が明示したパス。
    明示された場所(PathBuf),
    /// `rclone_executable`が省略された。PATH上の`rclone`を使うべきことを表す。
    PATH上の実行ファイル,
}

impl Rclone実行ファイルの場所 {
    pub(crate) fn 生成する(値: Option<String>) -> Self {
        match 値 {
            Some(パス文字列) => Self::明示された場所(PathBuf::from(パス文字列)),
            None => Self::PATH上の実行ファイル,
        }
    }
}
