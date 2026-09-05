//! `git lfs install`が登録するフック1種類分を表す値型。フックの名前と、その名前から
//! 決まる「標準の内容か」の判定を1つの型が持つ（グローバルCLAUDE.md「プリミティブ執着
//! 禁止はパス・テキスト・名前にも適用する」）。
//!
//! 前提: 判定の条件は実機の`git-lfs/3.5.1`で確かめた内容から決めた（推測で書かない）。
//! `git lfs install --local`が書くフックは、シバンの行・`command -v git-lfs`で始まる
//! 不在時の警告行・`git lfs <フック名> "$@"`の呼び出し行の3行だけで構成される。この3種
//! 以外の行が1行でもあれば、Git LFS以外の内容が混ざっているとみなす。
//!
//! 注意: `git lfs update --manual`はフックの検査ではなく、標準の内容を印字するだけで
//! 常に終了コード0を返す。検査を行う`git lfs update`はフックを書き換えるため、
//! `doctor`からは呼べない。そのため本ファイルが内容の判定を持つ。

#[repr(transparent)]
pub(crate) struct GitLfsフック(&'static str);

impl GitLfsフック {
    /// `git lfs install`が登録する4種類。
    pub(crate) fn 全種類() -> [Self; 4] {
        [Self("pre-push"), Self("post-checkout"), Self("post-commit"), Self("post-merge")]
    }

    pub(crate) fn 名前(&self) -> &'static str {
        self.0
    }

    /// 与えられたフックの本文が、Git LFSが書く標準の内容だけで構成されているかを判定する。
    pub(crate) fn 本文が標準の内容か(&self, 本文: &str) -> bool {
        let 呼び出し行の先頭 = format!("git lfs {}", self.0);
        let mut 呼び出し行があるか = false;
        for 行 in 本文.lines() {
            let 整えた行 = 行.trim();
            if 整えた行.is_empty() || 整えた行.starts_with('#') || 整えた行.starts_with("command -v git-lfs") {
                continue;
            }
            if 整えた行.starts_with(&呼び出し行の先頭) {
                呼び出し行があるか = true;
                continue;
            }
            return false;
        }
        呼び出し行があるか
    }
}
