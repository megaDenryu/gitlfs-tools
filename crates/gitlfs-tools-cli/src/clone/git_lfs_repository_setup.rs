//! 複製した作業ツリーに対して`git lfs`を子プロセスとして起動する外部境界。
//! フィルターの登録（`git lfs install --local`）と実体の取得（`git lfs pull`）の2つを持つ。
//! どちらも`--global`を使わず、対象のリポジトリだけに効く形を保つ。

use std::process::Command;

use crate::child_process_exit_code::子プロセスの終了コードを表す文字列を作る;
use crate::clone::error::複製エラー;
use crate::work_tree_root::作業ツリールート;

pub(crate) struct 対象リポジトリのGitLFS操作 {
    作業ツリー: 作業ツリールート,
}

impl 対象リポジトリのGitLFS操作 {
    pub(crate) fn 生成する(作業ツリー: 作業ツリールート) -> Self {
        Self { 作業ツリー }
    }

    /// このリポジトリだけでGit LFSのフィルターを有効にする。
    pub(crate) fn フィルターをこのリポジトリだけへ登録する(&self) -> Result<(), 複製エラー> {
        self.実行する(&["lfs", "install", "--local"])
            .map_err(|説明| 複製エラー::フィルター登録に失敗 { 説明 })
    }

    /// pointerを実体へ置き換える。agentの登録後に呼ぶ。
    pub(crate) fn 実体を取得する(&self) -> Result<(), 複製エラー> {
        self.実行する(&["lfs", "pull"]).map_err(|説明| 複製エラー::実体の取得に失敗 { 説明 })
    }

    pub(crate) fn 作業ツリーのルート(&self) -> &作業ツリールート {
        &self.作業ツリー
    }

    fn 実行する(&self, 引数: &[&str]) -> Result<(), String> {
        let 終了状態 = Command::new("git")
            .args(引数)
            .current_dir(self.作業ツリー.パス())
            .status()
            .map_err(|エラー| format!("gitコマンドを起動できませんでした: {エラー}"))?;
        if 終了状態.success() {
            Ok(())
        } else {
            Err(format!("終了コード{}で終わりました", 子プロセスの終了コードを表す文字列を作る(&終了状態)))
        }
    }
}
