//! 項目8: rcloneのremoteをlocal backend Aからlocal backend Bへ変更し、同じ
//! オブジェクトツリーを複製した後は、Git側を変更せず同じcommitを復元できることを
//! 確かめる。項目1〜6が積み上げた保管先(storage_a)とorigin・commit履歴をそのまま使う。

use std::ffi::OsStr;

use crate::acceptance_check::agent_binary::対象実行ファイルパス;
use crate::acceptance_check::object_storage_root::オブジェクト保管ルート;
use crate::acceptance_check::scenario_setup;
use crate::acceptance_check::scenario_state::{主鎖状態, 追跡ファイル名};
use crate::acceptance_check::test_payload::決定的な内容を作る;
use crate::acceptance_check::workspace::一時作業域;

pub fn 実行する(状態: &主鎖状態, 作業域: &一時作業域, 実行ファイル: &対象実行ファイルパス) -> Result<String, String> {
    if 状態.旧oid.is_empty() {
        return Err("項目1〜6が未完走のため、複製すべきオブジェクトが保管先にない".to_owned());
    }

    let storage_b = オブジェクト保管ルート::生成する(作業域.子パス("storage_b"));
    状態.storage_a.複製する(&storage_b)?;

    let pc_a2 = scenario_setup::模擬pcを組み立てる(作業域, "pc-a2", &storage_b, 実行ファイル)?;
    let 受信先 = 作業域.子パス("pc_a2_workdir");
    pc_a2
        .git実行(作業域.ルート(), &[("GIT_LFS_SKIP_SMUDGE", OsStr::new("1"))], &["clone", &状態.origin.to_string_lossy(), &受信先.to_string_lossy()])?
        .成功を要求する("PC A2のclone(smudge抑止)")?;
    pc_a2.エージェントを実行する(&受信先, &["install"])?.成功を要求する("PC A2のinstall")?;
    pc_a2.git実行(&受信先, &[], &["checkout", &状態.旧commit])?.成功を要求する("複製先backendからの旧commit checkout")?;

    let 期待 = 決定的な内容を作る(300_000, 1);
    let 実際 = std::fs::read(受信先.join(追跡ファイル名)).map_err(|失敗| format!("受信先の読み取りに失敗: {失敗}"))?;
    if 実際 != 期待 {
        return Err("複製した保管先backendからの取得内容が旧commitの内容と一致しない".to_owned());
    }

    Ok(format!("Git側({})を変更せず、複製先backendから同じcommitを復元できた", &状態.旧commit[..12]))
}
