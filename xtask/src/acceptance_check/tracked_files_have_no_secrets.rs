//! 項目9: repository履歴、tracked files、sample設定、テスト出力を検査しても認証情報と
//! PC固有の実pathを含まないことを確かめる。受入試験が作る一時fixtureではなく、この
//! git-lfs-rclone-storageリポジトリ自身が持つtracked filesを対象にする。
//!
//! 注意: 探す綴りをこのファイルへ直接書くと、このファイル自身が追跡対象になった時点で
//! 検査が自分の探している綴りを自分の中に見つけて不合格になる。走査対象からこのファイルを
//! 除外すると本物の秘密が入ったときに検出できなくなるため除外はせず、代わりに各パターンを
//! 断片へ分けて実行時にのみ結合する。断片単独ではどの綴りとも一致しないため、この説明
//! コメントを含め、探している綴りそのものはソースのどこにも連続した形で現れない。

use std::path::Path;
use std::process::Command;

pub fn 実行する() -> Result<String, String> {
    let 現在地 = std::env::current_dir().map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))?;
    let 追跡ファイル一覧 = git_ls_filesを実行する(&現在地)?;

    let mut 検出一覧 = Vec::new();
    for 相対パス in &追跡ファイル一覧 {
        let Ok(内容) = std::fs::read(現在地.join(相対パス)) else { continue };
        let Ok(文字列) = String::from_utf8(内容) else { continue };
        for パターン in 疑わしい部分文字列を組み立てる() {
            if 文字列.contains(パターン.as_str()) {
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

fn 疑わしい部分文字列を組み立てる() -> Vec<String> {
    vec![
        クライアント認証の秘密情報を示すキー名を組み立てる(),
        認証更新用トークンを示すキー名を組み立てる(),
        秘密鍵ファイルの見出しを組み立てる(),
        円記号区切りの利用者ディレクトリ表記を組み立てる(),
        斜線区切りの利用者ディレクトリ表記を組み立てる(),
    ]
}

fn クライアント認証の秘密情報を示すキー名を組み立てる() -> String {
    ["client", "_secret"].concat()
}

fn 認証更新用トークンを示すキー名を組み立てる() -> String {
    ["refresh", "_token"].concat()
}

fn 秘密鍵ファイルの見出しを組み立てる() -> String {
    ["BEGIN PRIVATE", " KEY"].concat()
}

fn 円記号区切りの利用者ディレクトリ表記を組み立てる() -> String {
    ["C:\\Us", "ers\\"].concat()
}

fn 斜線区切りの利用者ディレクトリ表記を組み立てる() -> String {
    ["C:/Us", "ers/"].concat()
}
