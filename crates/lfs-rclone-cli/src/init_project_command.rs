//! `init-project`サブコマンドの実行。対象リポジトリのルートへ`.large-assets.toml`の
//! 雛形を作る。既存ファイルを無言で上書きしない（Issue #8「雛形生成」節）。
//! `.gitattributes`は対象パターンの正本であるため、この雛形生成の対象に含めない
//! （Issue #2 7.2節）。

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use lfs_rclone_config::設定スキーマ版;
use lfs_rclone_domain::プロファイル名;

use crate::command_error::コマンド実行エラー;
use crate::git_repository::Gitリポジトリ;
use crate::project_config_template::プロジェクト設定雛形本文;

pub(crate) fn 雛形生成を実行する(プロファイル: プロファイル名) -> ExitCode {
    match 雛形を作る(&プロファイル) {
        Ok(パス) => {
            println!("プロジェクト設定の雛形を作成しました: {}", パス.display());
            ExitCode::SUCCESS
        }
        Err(エラー) => {
            eprintln!("プロジェクト設定の雛形作成に失敗しました: {エラー}");
            ExitCode::FAILURE
        }
    }
}

fn 雛形を作る(プロファイル: &プロファイル名) -> Result<PathBuf, コマンド実行エラー> {
    let リポジトリ = Gitリポジトリ::現在地から検出する()?;
    let ルート = リポジトリ.作業ツリーのルート()?;
    let 配置先 = ルート.プロジェクト設定ファイルパス();

    if 配置先.is_file() {
        return Err(コマンド実行エラー::プロジェクト設定ファイル既存 { パス: 配置先.display().to_string() });
    }

    let 本文 = プロジェクト設定雛形本文::生成する(設定スキーマ版::最新(), プロファイル);
    fs::write(&配置先, 本文.文字列表現())
        .map_err(|エラー| コマンド実行エラー::プロジェクト設定ファイル書き込み失敗 { 説明: エラー.to_string() })?;
    Ok(配置先)
}
