//! 項目9: repository履歴、tracked files、sample設定、テスト出力を検査しても認証情報と
//! PC固有の実pathを含まないことを確かめる。受入試験が作る一時fixtureではなく、この
//! git-lfs-rclone-storageリポジトリ自身が持つtracked filesを対象にする。

use std::path::Path;
use std::process::Command;

const 疑わしい部分文字列: [&str; 5] = ["client_secret", "refresh_token", "BEGIN PRIVATE KEY", "C:\\Users\\", "C:/Users/"];

pub fn 実行する() -> Result<String, String> {
    let 現在地 = std::env::current_dir().map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))?;
    let 追跡ファイル一覧 = git_ls_filesを実行する(&現在地)?;

    let mut 検出一覧 = Vec::new();
    for 相対パス in &追跡ファイル一覧 {
        let Ok(内容) = std::fs::read(現在地.join(相対パス)) else { continue };
        let Ok(文字列) = String::from_utf8(内容) else { continue };
        for パターン in 疑わしい部分文字列 {
            if 文字列.contains(パターン) {
                検出一覧.push(format!("{相対パス}: \"{パターン}\""));
            }
        }
    }

    if 検出一覧.is_empty() {
        Ok(format!("tracked files {}件を検査し、認証情報・実PCパスらしき文字列は見つからなかった", 追跡ファイル一覧.len()))
    } else {
        Err(format!("疑わしい記述が{}件見つかった: {}", 検出一覧.len(), 検出一覧.join(", ")))
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
