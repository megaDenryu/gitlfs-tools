//! `doctor`サブコマンドの実行。プロジェクト設定・PC設定・プロファイル解決・rcloneの
//! 起動可否・一時ディレクトリの作成可否・Git LFSの導入状況・Gitの各種設定・保管先への
//! 書き込み可否を順に確認し、不足があれば利用者が次に取るべき行動とともに報告する
//! （Issue #8「設定を検証する」節）。診断項目が増えても本体は一覧の組み立てに専念し、
//! 各項目の判定は名前を付けられる責務ごとに別ファイルへ分ける
//! （コード分割規約.md「1つの概念のファイルが3本以上になった」の昇格経路）。

use std::process::ExitCode;

use lfs_rclone_config::PCプロファイル;

use crate::diagnostic_finding::診断結果;
use crate::git_repository::Gitリポジトリ;
use crate::pc_config_location_resolution::pc設定の場所を解決する;
use crate::working_directory_resolution::作業ディレクトリを解決する;
use crate::{
    config_diagnostic, git_attributes_diagnostic, git_lfs_filter_diagnostic, git_lfs_installation_diagnostic,
    git_transfer_diagnostic, storage_reachability_diagnostic, storage_write_diagnostic, temp_directory_provisioning,
};

pub(crate) fn 検証を実行する() -> ExitCode {
    let 結果一覧 = 診断結果を集める();
    let 全て揃っているか = 結果一覧.iter().all(診断結果::揃っているか);

    for 結果 in &結果一覧 {
        for 行 in 結果.表示行一覧() {
            println!("{行}");
        }
    }

    if 全て揃っているか { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

fn 診断結果を集める() -> Vec<診断結果> {
    let mut 結果一覧 = Vec::new();
    結果一覧.push(git_lfs_installation_diagnostic::診断する());

    let 起点 = match 作業ディレクトリを解決する() {
        Ok(ディレクトリ) => ディレクトリ,
        Err(エラー) => {
            結果一覧.push(診断結果::不足から生成する("作業ディレクトリ", &エラー, "作業ディレクトリへのアクセス権を確認してください"));
            return 結果一覧;
        }
    };

    let pc設定の場所 = match pc設定の場所を解決する() {
        Ok(場所) => 場所,
        Err(エラー) => {
            結果一覧.push(診断結果::不足から生成する("PC設定の置き場所", &エラー, "この環境の標準設定ディレクトリを確認してください"));
            return 結果一覧;
        }
    };

    let 設定診断 = config_diagnostic::診断する(&起点, &pc設定の場所);
    結果一覧.extend(設定診断.結果一覧);

    結果一覧.push(storage_reachability_diagnostic::診断する(設定診断.プロファイル.as_ref()));
    結果一覧.push(一時ディレクトリを診断する(設定診断.プロファイル.as_ref()));
    結果一覧.push(storage_write_diagnostic::診断する(設定診断.プロファイル.as_ref()));

    let リポジトリ検出結果 = Gitリポジトリ::現在地から検出する();
    結果一覧.push(git_transfer_diagnostic::診断する(リポジトリ検出結果.as_ref()));
    結果一覧.push(git_lfs_filter_diagnostic::診断する(リポジトリ検出結果.as_ref()));
    結果一覧.push(git_attributes_diagnostic::診断する(リポジトリ検出結果.as_ref()));

    結果一覧
}

fn 一時ディレクトリを診断する(プロファイル: Option<&PCプロファイル>) -> 診断結果 {
    let 項目 = "一時ディレクトリの作成確認";
    let Some(プロファイル) = プロファイル else {
        return 診断結果::不足から生成する(項目, &"プロファイルが解決できていません", "先に設定の不足を解消してください");
    };
    match temp_directory_provisioning::一時保存先を作成する(プロファイル.一時ディレクトリ()) {
        Ok(()) => 診断結果::問題なし { 項目 },
        Err(エラー) => 診断結果::不足から生成する(項目, &エラー, "temp_directoryの権限・空き容量を確認してください"),
    }
}
