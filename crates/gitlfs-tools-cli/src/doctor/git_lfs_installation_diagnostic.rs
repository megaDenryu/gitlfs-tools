//! `doctor`の追加診断: `git lfs`自体がこの環境に導入されているかを確かめる。2台目以降の
//! PCで最初に踏む不足であり、`git lfs version`を子プロセスとして起動できるかどうかで
//! 判定する。Gitリポジトリの検出には依存しない（`git lfs version`はリポジトリの外でも
//! 実行できる）。

use std::process::{Command, Stdio};

use crate::child_process_exit_code::子プロセスの終了コードを表す文字列を作る;
use crate::doctor::finding::診断結果;

const 項目名: &str = "Git LFSの導入確認";
const 導入手順の案内: &str = "Git LFSを導入してください(https://git-lfs.com/)";

pub(crate) fn git_lfsの導入を診断する() -> 診断結果 {
    match Command::new("git").args(["lfs", "version"]).stdin(Stdio::null()).output() {
        Ok(出力) if 出力.status.success() => 診断結果::問題なし { 項目: 項目名 },
        Ok(出力) => {
            let 終了コード = 子プロセスの終了コードを表す文字列を作る(&出力.status);
            let 説明 = format!("git lfs versionが終了コード{終了コード}で失敗しました");
            診断結果::不足から生成する(項目名, &説明, 導入手順の案内)
        }
        Err(エラー) => 診断結果::不足から生成する(項目名, &エラー, 導入手順の案内),
    }
}
