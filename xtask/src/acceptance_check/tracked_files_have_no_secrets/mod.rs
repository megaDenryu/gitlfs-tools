//! 項目9: repository履歴、tracked files、sample設定、テスト出力を検査しても認証情報と
//! PC固有の実pathを含まないことを確かめる。受入試験が作る一時fixtureではなく、この
//! gitlfs-toolsリポジトリ自身が持つtracked filesを対象にする。
//!
//! 判定は「秘密を指すキー名が現れるか」ではなく「本物の秘密の値・本物のPC固有の実path
//! であるか」で行う。文書が秘密の扱いを説明するためにキー名へ言及する記述と、
//! `<ユーザー名>`のようなプレースホルダを含むpathは、どちらも秘密ではないため検出しない。
//!
//! 注意: 走査対象から特定のファイルを除外すると、そのファイルへ本物の秘密が入ったときに
//! 検出できなくなる。除外は行わず、検出の精度で誤検出を避ける。

mod personal_path;
mod secret_value;

use std::path::Path;
use std::process::Command;

use personal_path::実利用者ディレクトリの検出器;
use secret_value::秘密の値の検出器;

pub fn 実行する() -> Result<String, String> {
    let 現在地 =
        std::env::current_dir().map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))?;
    追跡ファイルの秘密検査器::実行環境から生成する()?.リポジトリを検査する(&現在地)
}

struct 追跡ファイルの秘密検査器 {
    秘密の値: 秘密の値の検出器,
    実利用者ディレクトリ: 実利用者ディレクトリの検出器,
}

impl 追跡ファイルの秘密検査器 {
    fn 実行環境から生成する() -> Result<Self, String> {
        Ok(Self {
            秘密の値: 秘密の値の検出器::既定(),
            実利用者ディレクトリ: 実利用者ディレクトリの検出器::実行環境から生成する()?,
        })
    }

    fn リポジトリを検査する(&self, リポジトリルート: &Path) -> Result<String, String> {
        let 追跡ファイル一覧 = git_ls_filesを実行する(リポジトリルート)?;

        let mut 検出一覧 = Vec::new();
        for 相対パス in &追跡ファイル一覧 {
            let Ok(内容) = std::fs::read(リポジトリルート.join(相対パス)) else { continue };
            let Ok(文字列) = String::from_utf8(内容) else { continue };
            for 説明 in self.内容から検出した記述を列挙する(&文字列) {
                検出一覧.push(format!("{相対パス}: {説明}"));
            }
        }

        if 検出一覧.is_empty() {
            Ok(format!(
                "tracked files {}件を検査し、値の入った認証情報と実PCパスは見つからなかった",
                追跡ファイル一覧.len()
            ))
        } else {
            Err(format!("疑わしい記述が{}件見つかった: {}", 検出一覧.len(), 検出一覧.join(", ")))
        }
    }

    fn 内容から検出した記述を列挙する(&self, 内容: &str) -> Vec<String> {
        let mut 説明一覧 = self.秘密の値.検出した記述を列挙する(内容);
        説明一覧.extend(self.実利用者ディレクトリ.検出した記述を列挙する(内容));
        説明一覧
    }
}

fn git_ls_filesを実行する(リポジトリルート: &Path) -> Result<Vec<String>, String> {
    let 出力 = Command::new("git")
        .args(["ls-files"])
        .current_dir(リポジトリルート)
        .output()
        .map_err(|失敗| format!("git ls-filesを起動できなかった: {失敗}"))?;
    if !出力.status.success() {
        return Err("git ls-filesが失敗した".to_owned());
    }
    Ok(String::from_utf8_lossy(&出力.stdout).lines().map(str::to_owned).collect())
}
