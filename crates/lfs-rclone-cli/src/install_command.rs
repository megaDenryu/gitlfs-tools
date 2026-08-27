//! `install`サブコマンドの実行。対象GitリポジトリへGit LFS custom transfer設定を
//! `--local`スコープで登録する。全リポジトリへの無条件適用を避けるため`--global`は
//! 使わない（CLAUDE.md「全Gitリポジトリへ無条件に適用してはならない」、Issue #2 7.3節）。
//! 既存の値がある場合も上書きし、変更なし・更新・新規のどれだったかを標準出力へ示す
//! （再実行しても同じ結果へ収束する、Git設定コマンドの一般的な慣習に合わせた）。

use std::process::ExitCode;

use crate::command_error::コマンド実行エラー;
use crate::git_repository::{Gitリポジトリ, 設定書き込み結果};
use crate::git_transfer_settings::Git転送設定;
use crate::install_target_path::登録する実行ファイルパス;

pub(crate) fn 導入を実行する(上書きパス: Option<登録する実行ファイルパス>) -> ExitCode {
    match 対象リポジトリへ設定を登録する(上書きパス) {
        Ok(()) => ExitCode::SUCCESS,
        Err(エラー) => {
            eprintln!("導入に失敗しました: {エラー}");
            ExitCode::FAILURE
        }
    }
}

fn 対象リポジトリへ設定を登録する(上書きパス: Option<登録する実行ファイルパス>) -> Result<(), コマンド実行エラー> {
    let 実行ファイルパス = match 上書きパス {
        Some(パス) => パス,
        None => 登録する実行ファイルパス::現在の実行ファイルから生成する()?,
    };
    let リポジトリ = Gitリポジトリ::現在地から検出する()?;
    let 設定 = Git転送設定::生成する(実行ファイルパス);

    println!("対象リポジトリへcustom transfer設定を登録します。");
    for (キー, 値) in 設定.キーと値の一覧() {
        let 結果 = リポジトリ.ローカル設定を書き込んで結果を返す(キー, &値)?;
        書き込み結果を表示する(キー, &値, &結果);
    }
    Ok(())
}

fn 書き込み結果を表示する(キー: &str, 値: &str, 結果: &設定書き込み結果) {
    match 結果 {
        設定書き込み結果::変更なし => println!("  {キー} = {値}（変更なし）"),
        設定書き込み結果::更新前 { 旧値 } => println!("  {キー} = {値}（{旧値}から更新）"),
        設定書き込み結果::新規 => println!("  {キー} = {値}（新規）"),
    }
}
