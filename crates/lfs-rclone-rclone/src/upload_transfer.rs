//! ローカルの検証済み入力を保管先の一時オブジェクトパスへ転送する外部境界(copyto)。

use std::ffi::OsString;
use std::path::Path;

use lfs_rclone_domain::{保管エラー, 保管先オブジェクトパス, Rcloneリモート名};

use crate::rclone_operation::Rclone操作;
use crate::rclone_process_runner::Rcloneプロセス実行器;

pub(crate) fn ローカル入力を一時オブジェクトパスへ転送する(
    実行器: &Rcloneプロセス実行器,
    リモート名: &Rcloneリモート名,
    ローカルパス: &Path,
    リモート宛先: &保管先オブジェクトパス,
) -> Result<(), 保管エラー> {
    let 宛先 = format!("{}:{}", リモート名.文字列表現(), リモート宛先.文字列表現());
    let 引数 = vec![
        OsString::from("copyto"),
        OsString::from("--ignore-times"),
        OsString::from("-q"),
        OsString::from("--stats"),
        OsString::from("0"),
        ローカルパス.as_os_str().to_os_string(),
        OsString::from(宛先),
    ];

    実行器.実行する(Rclone操作::アップロード転送, &引数)?;
    Ok(())
}
