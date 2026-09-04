//! `doctor`が「ダウンロードの一時ファイル置き場を作れるか」を確かめる。
//!
//! 確かめるのはPC設定の値ではなくリポジトリ側の実際の置き場所である。ここが
//! `<Git LFS保管ディレクトリ>/tmp/`の下にある限り、Git LFSが`complete`後に行う`rename`は
//! 同一ボリューム内で完結する（`git_lfs_storage_directory.rs`の注意書き）。

use std::path::Path;

use crate::diagnostic_finding::診断結果;
use crate::git_lfs_storage_directory::GitLFS保管ディレクトリ;
use crate::temp_directory_provisioning::一時保存先を作成する;

const 項目名: &str = "ダウンロード一時ディレクトリの作成確認";

pub(crate) fn 診断する(起点: &Path) -> 診断結果 {
    let 保管ディレクトリ = match GitLFS保管ディレクトリ::作業ディレクトリから問い合わせる(起点) {
        Ok(ディレクトリ) => ディレクトリ,
        Err(エラー) => {
            return 診断結果::不足から生成する(項目名, &エラー, "Gitリポジトリの作業ツリーの中で実行してください");
        }
    };

    match 一時保存先を作成する(&保管ディレクトリ.ダウンロード一時ディレクトリ()) {
        Ok(()) => 診断結果::問題なし { 項目: 項目名 },
        Err(エラー) => 診断結果::不足から生成する(項目名, &エラー, "リポジトリのGitディレクトリへの書き込み権限と空き容量を確認してください"),
    }
}
