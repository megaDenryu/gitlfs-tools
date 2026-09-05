//! Git LFSがobjectを置くディレクトリを`git`へ問い合わせ、その下のダウンロード一時
//! ディレクトリを導く外部境界。
//!
//! 注意: ダウンロードの一時ファイルは、`complete`でパスを返した時点でGit LFSの所有物に
//! なり、Git LFSはそれを`<Git LFS保管ディレクトリ>/objects/`へ`rename`で移す。Windowsの
//! `rename`はボリュームをまたげないため、一時ファイルがリポジトリと別のドライブにあると
//! 「cannot move the file to a different disk drive」で転送全体が失敗する。この型は
//! 一時ファイルの置き場所を保管先の設定から切り離し、必ず同じボリュームへ落とすためにある。
//!
//! 保管ディレクトリの綴りはGit LFS自身の規則に合わせる。既定は`<Gitディレクトリ>/lfs`で、
//! Git設定`lfs.storage`があればそれを使う（相対パスはGitディレクトリからの相対）。

use std::path::{Path, PathBuf};
use std::process::Command;

use gitlfs_tools_domain::一時ディレクトリ;

use crate::command_error::コマンド実行エラー;

/// agentが払い出す一時ファイルを置く子ディレクトリ名。Git LFS自身も`lfs/tmp`直下を
/// 使うため、双方の後片づけが互いのファイルへ触れないよう1階層分けて置く。
const エージェント専用の子ディレクトリ名: &str = "gitlfs-tools";

pub(crate) struct GitLFS保管ディレクトリ(PathBuf);

impl GitLFS保管ディレクトリ {
    pub(crate) fn 作業ディレクトリから問い合わせる(作業ディレクトリ: &Path) -> Result<Self, コマンド実行エラー> {
        let gitディレクトリ = git出力を取り出す(作業ディレクトリ, &["rev-parse", "--absolute-git-dir"])?;
        let 保管先の指定 = git出力を取り出す(作業ディレクトリ, &["config", "--get", "lfs.storage"]).ok();
        Ok(Self(保管ディレクトリを組み立てる(PathBuf::from(gitディレクトリ), 保管先の指定)))
    }

    /// agentがダウンロード先として払い出す一時ファイルの置き場所。`objects`と同じ親の下に
    /// あるため、Git LFSが行う`rename`が必ず同一ボリューム内で完結する。
    pub(crate) fn ダウンロード一時ディレクトリ(&self) -> 一時ディレクトリ {
        一時ディレクトリ::生成する(self.0.join("tmp").join(エージェント専用の子ディレクトリ名))
    }
}

fn 保管ディレクトリを組み立てる(gitディレクトリ: PathBuf, 保管先の指定: Option<String>) -> PathBuf {
    match 保管先の指定 {
        Some(指定) if !指定.is_empty() => {
            let 指定パス = PathBuf::from(指定);
            if 指定パス.is_absolute() { 指定パス } else { gitディレクトリ.join(指定パス) }
        }
        _ => gitディレクトリ.join("lfs"),
    }
}

fn git出力を取り出す(作業ディレクトリ: &Path, 引数: &[&str]) -> Result<String, コマンド実行エラー> {
    let 出力 = Command::new("git")
        .current_dir(作業ディレクトリ)
        .args(引数)
        .output()
        .map_err(|エラー| コマンド実行エラー::Gitコマンド起動失敗 { 説明: エラー.to_string() })?;
    if !出力.status.success() {
        return Err(コマンド実行エラー::Gitリポジトリ外);
    }
    Ok(String::from_utf8_lossy(&出力.stdout).trim().to_owned())
}
