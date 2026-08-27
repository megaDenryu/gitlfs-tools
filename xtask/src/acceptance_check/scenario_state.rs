//! 受入試験1〜6番が共有する状態。可変の途中経過をこの型のメソッドへ閉じ、自由関数へ
//! `&mut`で連鎖させない（グローバルCLAUDE.md「可変のドメイン状態を&mutで…通さない」）。
//! 各項目の実装は`pointer_only_commit.rs`から`repush_avoids_duplicate.rs`まで、項目ごとに
//! 1ファイルずつ続きの`impl`として持つ。

use std::path::PathBuf;

use crate::acceptance_check::object_storage_root::オブジェクト保管ルート;
use crate::acceptance_check::pc_environment::模擬PC;
use crate::acceptance_check::sha256_digest::ファイル指紋;

pub const プロファイル名: &str = "accept-v1";
pub const 追跡ファイル名: &str = "payload.bin";

pub struct 主鎖状態 {
    pub storage_a: オブジェクト保管ルート,
    pub origin: PathBuf,
    pub pc_a: 模擬PC,
    pub pc_b: 模擬PC,
    pub pc_a_workdir: PathBuf,
    pub pc_b_workdir: PathBuf,
    pub 旧oid: String,
    pub 新oid: String,
    pub 旧commit: String,
    pub 新commit: String,
    pub 旧オブジェクト指紋直後: Option<ファイル指紋>,
}

impl 主鎖状態 {
    pub fn 生成する(
        storage_a: オブジェクト保管ルート,
        origin: PathBuf,
        pc_a: 模擬PC,
        pc_b: 模擬PC,
        pc_a_workdir: PathBuf,
        pc_b_workdir: PathBuf,
    ) -> Self {
        Self {
            storage_a,
            origin,
            pc_a,
            pc_b,
            pc_a_workdir,
            pc_b_workdir,
            旧oid: String::new(),
            新oid: String::new(),
            旧commit: String::new(),
            新commit: String::new(),
            旧オブジェクト指紋直後: None,
        }
    }
}
