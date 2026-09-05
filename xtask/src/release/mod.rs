//! 版を上げた後に、検証を通してからタグを作りoriginへ送るまでを1コマンドへまとめる。
//!
//! このコマンドは版を書き換えない。版を上げる判断は人が`Cargo.toml`の
//! `[workspace.package]`の`version`を編集して行い、`release`はその版でタグを打つだけである。
//! タグを送ると`.github/workflows/release.yml`が動き、GitHubのReleaseが作られる。

mod git_work_tree;
mod publication_mode;
mod release_tag;
mod unmet_precondition;
mod workspace_version;

use crate::command_registry::サブコマンド;
use crate::verify_command::検証コマンド;
use git_work_tree::Git作業ツリー;
use publication_mode::タグ発行の仕方;
use release_tag::リリースタグ名;
use unmet_precondition::満たしていないリリースの前提;
use workspace_version::ワークスペースの版;

pub struct リリースタグ発行コマンド;

impl サブコマンド for リリースタグ発行コマンド {
    fn 名前(&self) -> &'static str {
        "release"
    }

    fn 説明(&self) -> &'static str {
        "検証を通し、Cargo.tomlの版でタグを作ってoriginへ送る(--dry-runで下見)"
    }

    fn 実行する(&self, 引数: &[String]) -> Result<(), String> {
        let 発行の仕方 = タグ発行の仕方::起動引数から解釈する(引数)?;
        let 版 = ワークスペースの版::リポジトリルートの設定ファイルから読む()?;
        let タグ名 = リリースタグ名::版から組み立てる(&版);
        let 作業ツリー = Git作業ツリー::カレントディレクトリを対象にする()?;
        eprintln!("== 発行の対象: 版 {} / タグ {} ==", 版.版の文字列(), タグ名.タグ名の文字列());

        let 前提 = 満たしていないリリースの前提::作業ツリーとタグ名から調べる(&作業ツリー, &タグ名)?;
        for 説明 in 前提.説明一覧() {
            eprintln!("前提を満たしていない: {説明}");
        }
        if !前提.すべて満たしているか() && !発行の仕方.満たしていない前提があっても続けるか() {
            return Err(format!("前提を{}件満たしていないため、タグを作らずに中止した", 前提.件数()));
        }

        eprintln!("== 検証: cargo xtask verify ==");
        検証コマンド.実行する(&[]).map_err(|失敗| format!("検証で失敗した。{失敗}"))?;

        match 発行の仕方 {
            タグ発行の仕方::下見だけ => 下見の結果を報告する(&タグ名, &前提),
            タグ発行の仕方::実際に発行する => タグを発行する(&作業ツリー, &タグ名),
        }
    }
}

fn 下見の結果を報告する(タグ名: &リリースタグ名, 前提: &満たしていないリリースの前提) -> Result<(), String> {
    eprintln!("== 下見のため、ここでタグを作らずに終える。実際の発行では次の2つを行う ==");
    eprintln!("  git tag {}", タグ名.タグ名の文字列());
    eprintln!("  git push origin {}", タグ名.タグ名の文字列());
    if 前提.すべて満たしているか() {
        return Ok(());
    }
    Err(format!("前提を{}件満たしていないため、この状態では発行できない", 前提.件数()))
}

fn タグを発行する(作業ツリー: &Git作業ツリー, タグ名: &リリースタグ名) -> Result<(), String> {
    作業ツリー.タグを作る(タグ名)?;
    eprintln!("タグを作った: {}", タグ名.タグ名の文字列());
    作業ツリー.タグをoriginへ送る(タグ名)?;
    eprintln!("タグをoriginへ送った: {}", タグ名.タグ名の文字列());
    eprintln!("GitHub Actionsのrelease.ymlがこのタグを起点にReleaseを作る");
    Ok(())
}
