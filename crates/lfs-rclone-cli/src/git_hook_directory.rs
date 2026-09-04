//! Gitがフックの実行ファイルを探すディレクトリを表す値型。`core.hooksPath`の設定で
//! 場所が変わるため、パスの導出をこの型のメソッドへ閉じ、呼び出し側で`join`を連ねさせない
//! （グローバルCLAUDE.md「役割の型は自分の配置を知る」）。

use std::path::PathBuf;

use crate::git_lfs_hook::GitLfsフック;

#[repr(transparent)]
pub(crate) struct Gitフック置き場(PathBuf);

impl Gitフック置き場 {
    pub(crate) fn 生成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// この置き場に置く、指定したフックのファイルの配置先。
    pub(crate) fn フックファイルパス(&self, フック: &GitLfsフック) -> PathBuf {
        self.0.join(フック.名前())
    }
}
