//! `doctor`の追加診断: このリポジトリで`git lfs install --local`によるフィルター登録が
//! 済んでいるかを確かめる。初回clone後の正式手順は`git lfs install --local`を含むが、
//! これを実行していないとGit LFSのフィルターが働かず、内容がpointerのままになる。
//!
//! 前提: `filter.lfs.required`は`git lfs install`が必ず書き込むキーであり、実機の
//! `git lfs 3.5.1`で`git lfs install --local`を実行して`filter.lfs.process`・
//! `filter.lfs.required`・`filter.lfs.clean`・`filter.lfs.smudge`の4キーが登録される
//! ことを確かめた上で選んだ（推測で書かない）。Gitリポジトリの検出は`doctor_command`が
//! 1回だけ行い、この関数はその結果を受け取る。

use crate::command_error::コマンド実行エラー;
use crate::diagnostic_finding::診断結果;
use crate::git_repository::Gitリポジトリ;

const 項目名: &str = "このリポジトリでのGit LFSフィルター登録確認";
const 確認キー: &str = "filter.lfs.required";

pub(crate) fn 診断する(リポジトリ検出結果: Result<&Gitリポジトリ, &コマンド実行エラー>) -> 診断結果 {
    let リポジトリ = match リポジトリ検出結果 {
        Ok(リポジトリ) => リポジトリ,
        Err(エラー) => return 診断結果::不足から生成する(項目名, エラー, "対象のGitリポジトリの中でdoctorを実行してください"),
    };

    if リポジトリ.ローカル設定を取得する(確認キー).is_some() {
        診断結果::問題なし { 項目: 項目名 }
    } else {
        診断結果::不足 {
            項目: 項目名,
            何が: "このリポジトリでGit LFSのフィルターが有効化されていません".to_owned(),
            どうすれば直るか: "git lfs install --localを実行してください".to_owned(),
        }
    }
}
