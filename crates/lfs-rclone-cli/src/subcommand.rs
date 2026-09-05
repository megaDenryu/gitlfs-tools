//! CLI引数から解釈した1件のサブコマンドと、その引数を表す判別共用体。
//! サブコマンド名の綴りは英語にする（Gitのサブコマンドと並んで使われるため）。

use lfs_rclone_domain::プロファイル名;

use crate::clone::source_url::複製元リポジトリURL;
use crate::clone::target_directory::複製先ディレクトリ;
use crate::install_target_path::登録する実行ファイルパス;
use crate::launch_argument_error::起動引数エラー;
use crate::object_check_scope::点検範囲;

pub(crate) enum サブコマンド {
    導入 { 実行ファイルパス: Option<登録する実行ファイルパス> },
    雛形生成 { プロファイル: プロファイル名 },
    検証,
    保管先の点検 { 範囲: 点検範囲 },
    複製 { 複製元: 複製元リポジトリURL, 複製先の指定: Option<複製先ディレクトリ> },
    ヘルプ,
}

impl サブコマンド {
    pub(crate) fn 解釈する(名前: &str, 残り引数: &[String]) -> Result<Self, 起動引数エラー> {
        match 名前 {
            "install" => 導入引数を解釈する(残り引数),
            "init-project" => 雛形生成引数を解釈する(残り引数),
            "doctor" => 引数なしを確かめる(名前, 残り引数).map(|()| Self::検証),
            "check-objects" => 点検引数を解釈する(残り引数),
            "clone" => 複製引数を解釈する(残り引数),
            "help" | "--help" | "-h" => 引数なしを確かめる(名前, 残り引数).map(|()| Self::ヘルプ),
            他 => Err(起動引数エラー::未知のサブコマンド { 名前: 他.to_owned() }),
        }
    }
}

fn 引数なしを確かめる(名前: &str, 残り引数: &[String]) -> Result<(), 起動引数エラー> {
    match 残り引数.first() {
        None => Ok(()),
        Some(余分) => Err(起動引数エラー::未知の引数 { サブコマンド: 名前.to_owned(), 名前: 余分.clone() }),
    }
}

/// `check-objects`の引数は`--all`の有無だけである。指定が無ければ現在のチェックアウトを
/// 点検する。
fn 点検引数を解釈する(引数: &[String]) -> Result<サブコマンド, 起動引数エラー> {
    let mut 範囲 = 点検範囲::現在のチェックアウト;
    for 項目 in 引数 {
        match 項目.as_str() {
            "--all" => 範囲 = 点検範囲::全履歴,
            他 => return Err(起動引数エラー::未知の引数 { サブコマンド: "check-objects".to_owned(), 名前: 他.to_owned() }),
        }
    }
    Ok(サブコマンド::保管先の点検 { 範囲 })
}

/// `clone`が受けるのは複製元のURLと、省略可能な複製先のディレクトリ名だけである
/// （`clone/command.rs`の判断4）。`git clone`のオプションは通さないため、`-`で始まる
/// 引数も余分な位置引数も失敗として扱う。
fn 複製引数を解釈する(引数: &[String]) -> Result<サブコマンド, 起動引数エラー> {
    let mut 位置引数 = Vec::new();
    for 項目 in 引数 {
        if 項目.starts_with('-') {
            return Err(起動引数エラー::未知の引数 { サブコマンド: "clone".to_owned(), 名前: 項目.clone() });
        }
        位置引数.push(項目.as_str());
    }
    match 位置引数.as_slice() {
        [] => Err(起動引数エラー::複製元のURLが必要),
        [複製元] => Ok(サブコマンド::複製 { 複製元: 複製元リポジトリURL::生成する(*複製元), 複製先の指定: None }),
        [複製元, 複製先] => Ok(サブコマンド::複製 {
            複製元: 複製元リポジトリURL::生成する(*複製元),
            複製先の指定: Some(複製先ディレクトリ::指定名から生成する(複製先)),
        }),
        [_, _, 余分, ..] => Err(起動引数エラー::未知の引数 { サブコマンド: "clone".to_owned(), 名前: (*余分).to_owned() }),
    }
}

fn 導入引数を解釈する(引数: &[String]) -> Result<サブコマンド, 起動引数エラー> {
    let mut 実行ファイルパス = None;
    let mut 残り = 引数.iter();
    while let Some(項目) = 残り.next() {
        match 項目.as_str() {
            "--path" => {
                let 値 = 残り.next().ok_or_else(|| 起動引数エラー::値が必要な引数 {
                    サブコマンド: "install".to_owned(),
                    引数名: "--path".to_owned(),
                })?;
                実行ファイルパス = Some(登録する実行ファイルパス::指定パスから生成する(値));
            }
            他 => return Err(起動引数エラー::未知の引数 { サブコマンド: "install".to_owned(), 名前: 他.to_owned() }),
        }
    }
    Ok(サブコマンド::導入 { 実行ファイルパス })
}

fn 雛形生成引数を解釈する(引数: &[String]) -> Result<サブコマンド, 起動引数エラー> {
    let mut プロファイル文字列 = None;
    let mut 残り = 引数.iter();
    while let Some(項目) = 残り.next() {
        match 項目.as_str() {
            "--profile" => {
                let 値 = 残り.next().ok_or_else(|| 起動引数エラー::値が必要な引数 {
                    サブコマンド: "init-project".to_owned(),
                    引数名: "--profile".to_owned(),
                })?;
                プロファイル文字列 = Some(値.clone());
            }
            他 => return Err(起動引数エラー::未知の引数 { サブコマンド: "init-project".to_owned(), 名前: 他.to_owned() }),
        }
    }
    let プロファイル文字列 = プロファイル文字列.ok_or(起動引数エラー::プロファイル名が必要)?;
    let プロファイル =
        プロファイル名::生成する(プロファイル文字列).map_err(|エラー| 起動引数エラー::プロファイル名が不正 { 説明: エラー.to_string() })?;
    Ok(サブコマンド::雛形生成 { プロファイル })
}
