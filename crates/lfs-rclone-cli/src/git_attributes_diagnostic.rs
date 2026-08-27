//! `doctor`の追加診断: `.gitattributes`にGit LFSの追跡対象パターンが1つ以上あるかを
//! 確かめる。`install`と`init-project`が両方成功しても、これが無いと大容量ファイルは
//! 1つも追跡されない。Gitリポジトリの検出は`doctor_command`が1回だけ行い、この関数は
//! その結果を受け取る。

use crate::command_error::コマンド実行エラー;
use crate::diagnostic_finding::診断結果;
use crate::git_repository::Gitリポジトリ;

const 項目名: &str = ".gitattributesのGit LFS追跡パターン確認";
const 追跡パターンの目印: &str = "filter=lfs";

pub(crate) fn 診断する(リポジトリ検出結果: Result<&Gitリポジトリ, &コマンド実行エラー>) -> 診断結果 {
    let リポジトリ = match リポジトリ検出結果 {
        Ok(リポジトリ) => リポジトリ,
        Err(エラー) => return 診断結果::不足から生成する(項目名, エラー, "対象のGitリポジトリの中でdoctorを実行してください"),
    };

    let ルート = match リポジトリ.作業ツリーのルート() {
        Ok(ルート) => ルート,
        Err(エラー) => return 診断結果::不足から生成する(項目名, &エラー, "対象のGitリポジトリの中でdoctorを実行してください"),
    };

    match std::fs::read_to_string(ルート.gitattributesファイルパス()) {
        Ok(本文) if 本文.lines().any(|行| 行.contains(追跡パターンの目印)) => 診断結果::問題なし { 項目: 項目名 },
        Ok(_) => 追跡パターン不足を報告する(),
        Err(エラー) if エラー.kind() == std::io::ErrorKind::NotFound => 追跡パターン不足を報告する(),
        Err(エラー) => 診断結果::不足から生成する(項目名, &エラー, ".gitattributesの読み取り権限を確認してください"),
    }
}

fn 追跡パターン不足を報告する() -> 診断結果 {
    診断結果::不足 {
        項目: 項目名,
        何が: ".gitattributesにGit LFSの追跡対象パターンがありません".to_owned(),
        どうすれば直るか:
            "git lfs track \"*.拡張子\"を実行してください(このコマンドは.gitattributesを書き換えます)".to_owned(),
    }
}
