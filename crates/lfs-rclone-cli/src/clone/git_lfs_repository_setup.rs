//! 複製した作業ツリーに対して`git lfs`を子プロセスとして起動する外部境界。
//! フィルターの登録（`git lfs install --local`）と実体の取得（`git lfs pull`）の2つを持つ。
//! どちらも`--global`を使わず、対象のリポジトリだけに効く形を保つ。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::child_process_exit_code::子プロセスの終了コードを表す文字列を作る;
use crate::clone::error::複製エラー;

pub(crate) struct 対象リポジトリのGitLFS操作 {
    作業ツリー: PathBuf,
}

impl 対象リポジトリのGitLFS操作 {
    pub(crate) fn 生成する(作業ツリー: impl Into<PathBuf>) -> Self {
        Self { 作業ツリー: 作業ツリー.into() }
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

    pub(crate) fn 作業ツリーのパス(&self) -> &Path {
        &self.作業ツリー
    }

    fn 実行する(&self, 引数: &[&str]) -> Result<(), String> {
        let 終了状態 = Command::new("git")
            .args(引数)
            .current_dir(&self.作業ツリー)
            .status()
            .map_err(|エラー| format!("gitコマンドを起動できませんでした: {エラー}"))?;
        if 終了状態.success() {
            Ok(())
        } else {
            Err(format!("終了コード{}で終わりました", 子プロセスの終了コードを表す文字列を作る(&終了状態)))
        }
    }
}
