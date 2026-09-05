//! `git clone`を子プロセスとして起動する外部境界。2つの環境変数をこの子プロセスの環境
//! だけへ渡す（`std::process::Command::env`）。利用者のシェルの環境変数は書き換えない
//! （Issue #11 判断1。`install-binary`がPATHを書き換えないのと同じ理由であり、edition 2024
//! では`std::env::set_var`が`unsafe fn`であるため、`unsafe_code = "forbid"`の本リポジトリは
//! そもそも自プロセスの環境変数を書き換えられない）。
//!
//! `GIT_CLONE_PROTECTION_ACTIVE=false`は安全の仕組みを切る指定であるため、切ることと
//! その理由を標準エラー出力へ書いてから渡す。黙って切ると、安全の仕組みが切られたことを
//! 利用者が知らないまま進む。

use std::process::Command;

use crate::child_process_exit_code::子プロセスの終了コードを表す文字列を作る;
use crate::clone::error::複製エラー;
use crate::clone::source_url::複製元リポジトリURL;
use crate::clone::target_directory::複製先ディレクトリ;

pub(crate) struct Git複製コマンド<'複製指定> {
    複製元: &'複製指定 複製元リポジトリURL,
    複製先: &'複製指定 複製先ディレクトリ,
}

impl<'複製指定> Git複製コマンド<'複製指定> {
    pub(crate) fn 生成する(複製元: &'複製指定 複製元リポジトリURL, 複製先: &'複製指定 複製先ディレクトリ) -> Self {
        Self { 複製元, 複製先 }
    }

    pub(crate) fn 実行する(&self) -> Result<(), 複製エラー> {
        渡す環境変数とその理由を知らせる();
        let 終了状態 = Command::new("git")
            .arg("clone")
            .arg(self.複製元.文字列表現())
            .arg(self.複製先.パス())
            .env("GIT_CLONE_PROTECTION_ACTIVE", "false")
            .env("GIT_LFS_SKIP_SMUDGE", "1")
            .status()
            .map_err(|エラー| 複製エラー::Gitコマンド起動失敗 { 説明: エラー.to_string() })?;

        if 終了状態.success() {
            Ok(())
        } else {
            Err(複製エラー::複製に失敗 {
                説明: format!("終了コード{}で終わりました", 子プロセスの終了コードを表す文字列を作る(&終了状態)),
            })
        }
    }
}

fn 渡す環境変数とその理由を知らせる() {
    eprintln!("git cloneの子プロセスへGIT_CLONE_PROTECTION_ACTIVE=falseを渡します。");
    eprintln!("  この保護は、cloneの最中に出所不明のフックが有効化されて実行されることを防ぐ仕組みです。");
    eprintln!("  ここで有効化されるのはGit LFS自身が登録する正規のフックであるため、この抑止で危険は増えません。");
    eprintln!("  この指定はgitの子プロセスの環境だけに効きます。利用者のシェルの環境変数は書き換えません。");
    eprintln!("あわせてGIT_LFS_SKIP_SMUDGE=1を渡し、agentの登録前にGit LFSが実体を取りに行く動作を止めます。");
}
