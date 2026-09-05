//! `doctor`の追加診断: このリポジトリのGit LFSフックに、Git LFS以外の内容が混ざって
//! いないかを確かめる。既にGit LFSを別の方式で使っていたリポジトリでは、実体の送信を
//! 止める`GIT_LFS_SKIP_PUSH=1`のような行を`pre-push`へ書き足していることがあり、その
//! 状態では`git lfs install --local`が`Hook already exists: pre-push`と表示して終了
//! コード2で止まる。導入の1行目で止まる不足であるため、`doctor`が導入の前に検出する。
//!
//! 前提: フックが1つも無い状態は合格として扱う。`git lfs install --local`をこれから
//! 実行する正常な状態であるためである。Gitリポジトリの検出は`doctor::command`が1回だけ
//! 行い、この関数はその結果を受け取る。

use crate::command_error::コマンド実行エラー;
use crate::doctor::finding::診断結果;
use crate::git_lfs_hook::GitLfsフック;
use crate::git_repository::Gitリポジトリ;

const 項目名: &str = "このリポジトリのGit LFSフックが標準の内容かの確認";
const リポジトリ外の案内: &str = "対象のGitリポジトリの中でdoctorを実行してください";

pub(crate) fn このリポジトリのgit_lfsフックの内容を診断する(リポジトリ検出結果: Result<&Gitリポジトリ, &コマンド実行エラー>) -> 診断結果 {
    let リポジトリ = match リポジトリ検出結果 {
        Ok(リポジトリ) => リポジトリ,
        Err(エラー) => return 診断結果::不足から生成する(項目名, エラー, リポジトリ外の案内),
    };

    let 置き場 = match リポジトリ.フック置き場を取得する() {
        Ok(置き場) => 置き場,
        Err(エラー) => return 診断結果::不足から生成する(項目名, &エラー, リポジトリ外の案内),
    };

    let mut 他の内容が混ざったフック名一覧 = Vec::new();
    for フック in GitLfsフック::全種類() {
        match std::fs::read_to_string(置き場.フックファイルパス(&フック)) {
            Ok(本文) if フック.本文が標準の内容か(&本文) => {}
            Ok(_) => 他の内容が混ざったフック名一覧.push(フック.名前()),
            Err(エラー) if エラー.kind() == std::io::ErrorKind::NotFound => {}
            Err(エラー) => {
                return 診断結果::不足から生成する(項目名, &エラー, "フックファイルの読み取り権限を確認してください");
            }
        }
    }

    if 他の内容が混ざったフック名一覧.is_empty() {
        診断結果::問題なし { 項目: 項目名 }
    } else {
        混入を報告する(&他の内容が混ざったフック名一覧)
    }
}

fn 混入を報告する(フック名一覧: &[&str]) -> 診断結果 {
    診断結果::不足 {
        項目: 項目名,
        何が: format!("フック{}にGit LFS以外の内容が混ざっています", フック名一覧.join("と")),
        どうすれば直るか: "元の方式へ戻せるように旧フックの内容を控えた上で、git lfs update --forceを実行して\
             Git LFSの標準の内容へ上書きしてください"
            .to_owned(),
    }
}
