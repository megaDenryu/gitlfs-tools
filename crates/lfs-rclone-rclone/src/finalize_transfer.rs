//! 保管先の一時オブジェクトパスから最終オブジェクトパスへ移す外部境界(moveto)。
//!
//! 注意: `moveto`は宛先に既存のファイルがあれば上書きする。同一OIDの同時アップロードが
//! 競合しても、この上書きにより最終的に1つの内容へ収束する。競合後にサイズが一致するかの
//! 判定は呼び出し側（`rclone_object_storage.rs`）が転送後の再確認で行う。

use std::ffi::OsString;

use lfs_rclone_domain::{保管エラー, 保管先オブジェクトパス, Rcloneリモート名};

use crate::rclone_operation::Rclone操作;
use crate::rclone_process_runner::Rcloneプロセス実行器;

pub(crate) fn 一時オブジェクトパスから最終オブジェクトパスへ移す(
    実行器: &Rcloneプロセス実行器,
    リモート名: &Rcloneリモート名,
    一時パス: &保管先オブジェクトパス,
    最終パス: &保管先オブジェクトパス,
) -> Result<(), 保管エラー> {
    let 移動元 = format!("{}:{}", リモート名.文字列表現(), 一時パス.文字列表現());
    let 移動先 = format!("{}:{}", リモート名.文字列表現(), 最終パス.文字列表現());
    let 引数 = vec![
        OsString::from("moveto"),
        OsString::from("--ignore-times"),
        OsString::from("-q"),
        OsString::from("--stats"),
        OsString::from("0"),
        OsString::from(移動元),
        OsString::from(移動先),
    ];

    実行器.実行する(Rclone操作::最終化転送, &引数)?;
    Ok(())
}
