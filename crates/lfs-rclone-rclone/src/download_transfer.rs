//! 保管先の最終オブジェクトパスからローカルの一時ファイルパスへ転送する外部境界(copyto)。

use std::ffi::OsString;

use lfs_rclone_domain::{一時ファイルパス, 保管エラー, 保管先オブジェクトパス, Rcloneリモート名};

use crate::rclone_operation::Rclone操作;
use crate::rclone_process_runner::Rcloneプロセス実行器;

/// 呼び出し側が用意した一時ファイルパスへ転送する。既存のファイルを無言で上書きしない
/// （Issue #2 4節）。
pub(crate) fn 最終オブジェクトを一時ファイルパスへ転送する(
    実行器: &Rcloneプロセス実行器,
    リモート名: &Rcloneリモート名,
    リモート元: &保管先オブジェクトパス,
    保存先: &一時ファイルパス,
) -> Result<(), 保管エラー> {
    if 保存先.パス().exists() {
        return Err(保管エラー::ローカル入出力 {
            説明: format!("ダウンロード先が既に存在します: {}", 保存先.パス().display()),
        });
    }

    let 転送元 = format!("{}:{}", リモート名.文字列表現(), リモート元.文字列表現());
    let 引数 = vec![
        OsString::from("copyto"),
        OsString::from("--ignore-times"),
        OsString::from("-q"),
        OsString::from("--stats"),
        OsString::from("0"),
        OsString::from(転送元),
        保存先.パス().as_os_str().to_os_string(),
    ];

    実行器.実行する(Rclone操作::ダウンロード転送, &引数)?;
    Ok(())
}
