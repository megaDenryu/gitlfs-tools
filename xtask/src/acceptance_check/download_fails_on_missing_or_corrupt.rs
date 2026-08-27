//! 項目7: 外部のオブジェクトを削除または破損させた試験で、downloadが明示的に失敗し、
//! 別の内容を作業treeへ置かないことを確かめる。項目1〜6のPC A/PC B/保管先とは独立した
//! 保管先とGitリポジトリを使い、既に完走した項目1〜6の状態を壊さない。

use std::ffi::OsStr;
use std::path::Path;

use crate::acceptance_check::agent_binary::対象実行ファイルパス;
use crate::acceptance_check::lfs_pointer;
use crate::acceptance_check::object_storage_root::オブジェクト保管ルート;
use crate::acceptance_check::pc_environment::模擬PC;
use crate::acceptance_check::scenario_setup;
use crate::acceptance_check::scenario_state::プロファイル名;
use crate::acceptance_check::test_payload::決定的な内容を作る;
use crate::acceptance_check::workspace::一時作業域;

pub fn 実行する(作業域: &一時作業域, 実行ファイル: &対象実行ファイルパス) -> Result<String, String> {
    let storage_c = オブジェクト保管ルート::生成する(作業域.子パス("storage_c"));
    let pc_c = scenario_setup::模擬pcを組み立てる(作業域, "pc-c", &storage_c, 実行ファイル)?;
    let origin_c = 作業域.子パス("origin_c.git");
    scenario_setup::裸リポジトリを作る(&pc_c, 作業域.ルート(), &origin_c)?;

    let (oid_削除, oid_破損) = 送出元リポジトリを準備する(作業域, &pc_c, &origin_c)?;

    let 受信先 = 作業域.子パス("repo_c2_workdir");
    pc_c.git実行(作業域.ルート(), &[("GIT_LFS_SKIP_SMUDGE", OsStr::new("1"))], &["clone", &origin_c.to_string_lossy(), &受信先.to_string_lossy()])?
        .成功を要求する("受信側のclone(smudge抑止)")?;
    pc_c.エージェントを実行する(&受信先, &["install"])?.成功を要求する("受信側のinstall")?;

    storage_c.オブジェクトを削除する(&oid_削除)?;
    storage_c.オブジェクトを破損させる(&oid_破損)?;

    let pull結果 = pc_c.git実行(&受信先, &[], &["lfs", "pull"])?;
    if pull結果.成功したか {
        return Err("削除・破損済みのオブジェクトがあるのにgit lfs pullが成功してしまった".to_owned());
    }

    working_treeが書き換わっていないか確認する(&受信先, "payload_del.bin")?;
    working_treeが書き換わっていないか確認する(&受信先, "payload_corrupt.bin")?;

    Ok(format!(
        "削除・破損させた2件ともgit lfs pullが明示的に失敗し(標準エラー出力: {})、working treeへ別内容は置かれなかった",
        pull結果.標準エラー出力.trim()
    ))
}

fn 送出元リポジトリを準備する(作業域: &一時作業域, pc: &模擬PC, origin: &Path) -> Result<(String, String), String> {
    let repo = 作業域.子パス("repo_c_workdir");
    pc.git実行(作業域.ルート(), &[], &["-c", "init.defaultBranch=main", "init", &repo.to_string_lossy()])?.成功を要求する("送出側のgit init")?;
    pc.git実行(&repo, &[], &["lfs", "install", "--local"])?.成功を要求する("送出側のgit lfs install --local")?;
    pc.git実行(&repo, &[], &["lfs", "track", "*.bin"])?.成功を要求する("送出側のgit lfs track")?;
    pc.エージェントを実行する(&repo, &["init-project", "--profile", プロファイル名])?.成功を要求する("送出側のinit-project")?;
    pc.エージェントを実行する(&repo, &["install"])?.成功を要求する("送出側のinstall")?;

    std::fs::write(repo.join("payload_del.bin"), 決定的な内容を作る(50_000, 11)).map_err(|失敗| format!("payload_del.binを書き込めなかった: {失敗}"))?;
    std::fs::write(repo.join("payload_corrupt.bin"), 決定的な内容を作る(50_000, 22)).map_err(|失敗| format!("payload_corrupt.binを書き込めなかった: {失敗}"))?;
    pc.git実行(&repo, &[], &["add", "-A"])?.成功を要求する("送出側のgit add")?;
    pc.コミットする(&repo, "項目7: 削除・破損試験用の2ファイル")?.成功を要求する("送出側のcommit")?;
    pc.git実行(&repo, &[], &["remote", "add", "origin", &origin.to_string_lossy()])?.成功を要求する("送出側のorigin登録")?;
    pc.git実行(&repo, &[], &["push", "origin", "main"])?.成功を要求する("送出側からのpush")?;

    let oid_削除 = pointerからoidを取り出す(pc, &repo, "payload_del.bin")?;
    let oid_破損 = pointerからoidを取り出す(pc, &repo, "payload_corrupt.bin")?;
    Ok((oid_削除, oid_破損))
}

fn pointerからoidを取り出す(pc: &模擬PC, repo: &Path, ファイル名: &str) -> Result<String, String> {
    let 出力 = pc.git実行(repo, &[], &["show", &format!("HEAD:{ファイル名}")])?.成功を要求する("commit blobの取得")?;
    let (oid, _size) = lfs_pointer::oidとサイズを取り出す(&出力.標準出力)?;
    Ok(oid)
}

fn working_treeが書き換わっていないか確認する(受信先: &Path, ファイル名: &str) -> Result<(), String> {
    let 対象 = 受信先.join(ファイル名);
    let 内容 = std::fs::read(&対象).map_err(|失敗| format!("{}を読み取れなかった: {失敗}", 対象.display()))?;
    let 先頭 = String::from_utf8_lossy(&内容[..内容.len().min(64)]);
    if lfs_pointer::pointer形式か(&先頭) {
        Ok(())
    } else {
        Err(format!("{}が想定外の内容へ書き換わっていた({}バイト)", 対象.display(), 内容.len()))
    }
}
