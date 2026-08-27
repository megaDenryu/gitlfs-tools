//! `doctor`がGit側のcustom transfer設定を検査する。`install`が書き込む4キー
//! （`git_transfer_settings.rs`参照）の存在だけを確認し、値の突き合わせはしない
//! （値の是正は`install`の再実行そのもので直るため、doctorは「登録済みか」だけを見る）。
//! Gitリポジトリの検出は`doctor_command`が1回だけ行い、この関数はその結果を受け取る
//! （グローバルCLAUDE.md「暗黙のグローバル依存を関数の奥で直叩きしない」。境界を
//! 呼び出し元へ1箇所集約する）。

use crate::command_error::コマンド実行エラー;
use crate::diagnostic_finding::診断結果;
use crate::git_repository::Gitリポジトリ;
use crate::git_transfer_settings::設定キー一覧;

const 項目名: &str = "Gitのcustom transfer設定";

pub(crate) fn 診断する(リポジトリ検出結果: Result<&Gitリポジトリ, &コマンド実行エラー>) -> 診断結果 {
    let リポジトリ = match リポジトリ検出結果 {
        Ok(リポジトリ) => リポジトリ,
        Err(エラー) => return 診断結果::不足から生成する(項目名, エラー, "対象のGitリポジトリの中でdoctorを実行してください"),
    };

    let 未設定: Vec<&str> = 設定キー一覧.into_iter().filter(|キー| リポジトリ.ローカル設定を取得する(キー).is_none()).collect();

    if 未設定.is_empty() {
        診断結果::問題なし { 項目: 項目名 }
    } else {
        診断結果::不足 {
            項目: 項目名,
            何が: format!("次のGit設定が未登録です: {}", 未設定.join(", ")),
            どうすれば直るか: "git-lfs-rclone-storage installを実行してください".to_owned(),
        }
    }
}
