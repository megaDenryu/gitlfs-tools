//! Git LFSの利用者PC1台分を模擬するサービス。孤立したglobal git設定ファイルと本
//! エージェントのPC設定ディレクトリを保持し、git起動をメソッドとして提供する
//! （グローバルCLAUDE.md「サービスの依存保持とコールバックの限定」。呼び出し側へ
//! 環境変数の組み立てを持参させない）。対象実行ファイルの起動は`pc_environment_agent_ops.rs`
//! が続きの`impl`として持つ（叩く相手がgitか本エージェント自身かという別の責務のため）。

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::acceptance_check::agent_binary::対象実行ファイルパス;
use crate::acceptance_check::pc_config_dir::PC設定ディレクトリ;
use crate::acceptance_check::process_output::子プロセス出力;

pub struct 模擬PC {
    表示名: String,
    分離global設定ファイル: PathBuf,
    pc設定: PC設定ディレクトリ,
    設定に残した一時ディレクトリ: PathBuf,
    実行ファイル: 対象実行ファイルパス,
}

impl 模擬PC {
    pub fn 生成する(
        表示名: impl Into<String>,
        分離global設定ファイル: PathBuf,
        pc設定: PC設定ディレクトリ,
        設定に残した一時ディレクトリ: PathBuf,
        実行ファイル: 対象実行ファイルパス,
    ) -> Self {
        Self { 表示名: 表示名.into(), 分離global設定ファイル, pc設定, 設定に残した一時ディレクトリ, 実行ファイル }
    }

    pub fn pc設定(&self) -> &PC設定ディレクトリ {
        &self.pc設定
    }

    /// このPCのconfig.tomlへ書いた`temp_directory`。agentはもうこの項目を読まないため、
    /// 試験は「このディレクトリが作られていないこと」を使われていないことの証拠にする。
    pub fn 設定に残した一時ディレクトリ(&self) -> &Path {
        &self.設定に残した一時ディレクトリ
    }

    pub fn 実行ファイル(&self) -> &対象実行ファイルパス {
        &self.実行ファイル
    }

    /// `git`をこのPCの孤立globalconfigの下で実行する。実ユーザーのglobal設定は変更しない
    /// （`GIT_CONFIG_GLOBAL`環境変数、既存の`install_command_test.rs`と同じ手法）。
    ///
    /// 注意1: `push`・`pull`・`checkout`・`clone`はgitの内部からGit LFSのhookやsmudge
    /// filterを介して対象実行ファイルを子孫プロセスとして起動しうる。対象実行ファイルは
    /// `LFS_RCLONE_PC_CONFIG_DIR`が無いと実ユーザーの標準設定ディレクトリを見に行こうと
    /// してしまうため、`git`を起動するすべての経路でこの環境変数を渡す。
    ///
    /// 注意2: machine単位の`git lfs install --skip-repo`がglobal templateへpost-checkout
    /// 等のhookを登録するため、以後の`git clone`はGitの「clone中に出所不明なhookが有効化
    /// された」保護に引っかかる。これはgit-lfsの正規のhookであり、この試験は隔離した
    /// 一時環境の中だけで完結するため、保護を無効化してよい
    /// （`GIT_CLONE_PROTECTION_ACTIVE=false`、Gitのエラーメッセージが案内する回避策）。
    pub fn git実行(&self, 作業ディレクトリ: &Path, 追加環境変数: &[(&str, &OsStr)], 引数: &[&str]) -> Result<子プロセス出力, String> {
        let mut コマンド = Command::new("git");
        コマンド
            .current_dir(作業ディレクトリ)
            .env("GIT_CONFIG_GLOBAL", &self.分離global設定ファイル)
            .env("LFS_RCLONE_PC_CONFIG_DIR", self.pc設定.パス())
            .env("GIT_CLONE_PROTECTION_ACTIVE", "false")
            .args(引数);
        for (キー, 値) in 追加環境変数 {
            コマンド.env(キー, 値);
        }
        起動して結果を包む(&mut コマンド, "git", 引数)
    }

    /// コミットの著者・コミッタをこのPCの表示名で固定して`git commit`する。
    pub fn コミットする(&self, 作業ディレクトリ: &Path, メッセージ: &str) -> Result<子プロセス出力, String> {
        let メール = format!("{}@example.invalid", self.表示名);
        self.git実行(
            作業ディレクトリ,
            &[
                ("GIT_AUTHOR_NAME", OsStr::new(&self.表示名)),
                ("GIT_AUTHOR_EMAIL", OsStr::new(&メール)),
                ("GIT_COMMITTER_NAME", OsStr::new(&self.表示名)),
                ("GIT_COMMITTER_EMAIL", OsStr::new(&メール)),
            ],
            &["commit", "-m", メッセージ],
        )
    }
}

pub(crate) fn 起動して結果を包む(コマンド: &mut Command, 対象名: &str, 引数: &[&str]) -> Result<子プロセス出力, String> {
    let 出力 = コマンド.output().map_err(|失敗| format!("{対象名}を起動できなかった(引数: {}): {失敗}", 引数.join(" ")))?;
    Ok(子プロセス出力 {
        成功したか: 出力.status.success(),
        標準出力: String::from_utf8_lossy(&出力.stdout).into_owned(),
        標準エラー出力: String::from_utf8_lossy(&出力.stderr).into_owned(),
    })
}
