//! `check-objects`サブコマンドの実行（Issue #12）。Git LFSが参照するオブジェクトが保管先に
//! 実在するかを突き合わせ、欠けているものを一覧で示す。
//!
//! `doctor`とは目的が違う。`doctor`の失敗は「設定を直せ」を意味し、この点検の失敗は
//! 「実体を送り直せ」を意味する。同じコマンドが2種類の行動を要求すると、出力を読んだ人が
//! 何をすべきか迷うため、別のサブコマンドへ分けている。

use std::process::ExitCode;

use gitlfs_tools_transfer::保管先オブジェクト在否点検サービス;

use crate::check_objects_output::点検結果の表示;
use crate::command_error::コマンド実行エラー;
use crate::git_lfs_file_listing::GitLFS追跡ファイル一覧;
use crate::object_check_scope::点検範囲;
use crate::pc_config_location_resolution::pc設定の場所を解決する;
use crate::profile_resolution::プロファイル解決に使う設定の置き場所;
use crate::storage_assembly::起動確認を済ませた保管庫を組み立てる;
use crate::working_directory_resolution::作業ディレクトリを解決する;

pub(crate) fn 保管先の点検を実行する(範囲: 点検範囲) -> ExitCode {
    match 点検して表示を作る(範囲) {
        Ok(表示) => {
            for 行 in 表示.表示行一覧() {
                println!("{行}");
            }
            if 表示.全て保管先に在るか() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Err(エラー) => {
            eprintln!("保管先の点検を実行できませんでした: {エラー}");
            ExitCode::FAILURE
        }
    }
}

fn 点検して表示を作る(範囲: 点検範囲) -> Result<点検結果の表示, コマンド実行エラー> {
    let 起点 = 作業ディレクトリを解決する().map_err(|エラー| コマンド実行エラー::設定の解決失敗 {
        説明: format!("作業ディレクトリを取得できませんでした: {エラー}"),
    })?;
    let pc設定の場所 = pc設定の場所を解決する().map_err(|エラー| コマンド実行エラー::設定の解決失敗 {
        説明: format!("PC設定の場所を解決できませんでした: {エラー}"),
    })?;

    let プロファイル = プロファイル解決に使う設定の置き場所::生成する(起点, pc設定の場所)
        .論理プロファイルを解決する()
        .map_err(|エラー| コマンド実行エラー::設定の解決失敗 { 説明: エラー.to_string() })?;
    let 保管庫 = 起動確認を済ませた保管庫を組み立てる(&プロファイル)
        .map_err(|エラー| コマンド実行エラー::保管先の準備失敗 { 説明: エラー.to_string() })?;

    let 追跡ファイル一覧 = GitLFS追跡ファイル一覧::作業ディレクトリへ問い合わせる(範囲)?;
    let 報告 = 保管先オブジェクト在否点検サービス::生成する(保管庫)
        .点検する(&追跡ファイル一覧.点検対象一覧へ変換する())
        .map_err(|エラー| コマンド実行エラー::保管先の点検失敗 { 説明: エラー.to_string() })?;

    Ok(点検結果の表示::生成する(範囲, 追跡ファイル一覧, 報告))
}
