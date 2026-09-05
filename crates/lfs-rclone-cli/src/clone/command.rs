//! `clone`サブコマンドの実行。複製・フィルターの登録・agentの登録・論理プロファイルの
//! 確認・実体の取得を順に行い、初回cloneの手順を1コマンドへまとめる（Issue #11）。
//!
//! Issue #11の「判断が要ること」に対して選んだ形と理由を、ここへ記録する。
//!
//! 判断1: `GIT_CLONE_PROTECTION_ACTIVE=false`は設定するが、設定することと理由を標準
//! エラー出力へ書く。黙って切ると、安全の仕組みが切られたことを利用者が知らないまま進む。
//! 既定で切らずに失敗したときだけ促す形は、利用者の手数が減らず1コマンド化の目的を果たさない。
//! 環境変数は`git clone`の子プロセスの環境へだけ渡す（`git_clone_process.rs`）。
//!
//! 判断2: 途中で失敗しても、複製した作業ツリーを消さない。消す操作は破壊的であり、利用者が
//! 既に書き込んだものを巻き添えにする。`git clone`自体が失敗した場合の後始末は`git`自身が
//! 行うため、それに任せる。`git clone`が成功して後段が失敗した場合は、どこへ複製したかと、
//! そのリポジトリで次に何をすればよいかを標準エラー出力へ書く。
//!
//! 判断3: `git lfs pull`の前に、論理プロファイルの解決だけを確かめる。`doctor`の12項目
//! すべては冗長である（`install`で整えた直後であり、フックと`.gitattributes`の状態は
//! 分かっている）。一方「複製したリポジトリの`.large-assets.toml`が指す論理プロファイル名が
//! このPCのPC設定に無い」は、他人のリポジトリを初めて取るときに必ず起きる詰まり方であり、
//! そのまま`git lfs pull`を走らせるとGit LFS越しの分かりにくい失敗になる。
//!
//! 判断4: 引数は複製元のURLと省略可能なディレクトリ名の2つだけを受け、`git clone`の他の
//! 引数は通さない。通す形にすると、agentが`git clone`の引数体系を実装することになる。
//! `--branch`や`--depth`が要る利用者は、`_doc/利用/プロジェクト導入.md`に残した従来の
//! 4手順を使う。複製先は`git`任せにせず必ず明示して渡す。後続の3つの処理をその場所で
//! 動かすため、場所を確実に知る必要があるからである。既に存在する空でないディレクトリを
//! 指定された場合は`git`自身が`fatal: destination path ... already exists and is not an
//! empty directory.`を出して終了コード128で止まるため（実測）、agentは独自の判定を持たず、
//! `git`のメッセージをそのまま利用者へ見せて失敗する。

use std::process::ExitCode;

use crate::clone::error::複製エラー;
use crate::clone::git_clone_process::Git複製コマンド;
use crate::clone::git_lfs_repository_setup::対象リポジトリのGitLFS操作;
use crate::clone::source_url::複製元リポジトリURL;
use crate::clone::target_directory::複製先ディレクトリ;
use crate::git_command_directory::Gitコマンドの実行場所;
use crate::install_command;
use crate::pc_config_location_resolution::pc設定の場所を解決する;
use crate::profile_resolution::プロファイル解決に使う設定の置き場所;

pub(crate) fn 複製を実行する(複製元: 複製元リポジトリURL, 複製先の指定: Option<複製先ディレクトリ>) -> ExitCode {
    let 複製先 = match 複製先を決める(&複製元, 複製先の指定) {
        Ok(先) => 先,
        Err(エラー) => {
            eprintln!("{エラー}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!("複製先: {}", 複製先.表示用の綴り());

    if let Err(エラー) = Git複製コマンド::生成する(&複製元, &複製先).実行する() {
        eprintln!("{エラー}");
        return ExitCode::FAILURE;
    }

    let 操作 = 対象リポジトリのGitLFS操作::生成する(複製先.パス());
    match 複製した作業ツリーを使える状態にする(&操作) {
        Ok(()) => {
            println!("cloneが完了しました: {}", 複製先.表示用の綴り());
            ExitCode::SUCCESS
        }
        Err(エラー) => {
            eprintln!("{エラー}");
            エラー.続けて示す案内を表示する();
            複製した作業ツリーの場所と続きの手順を知らせる(&複製先);
            ExitCode::FAILURE
        }
    }
}

fn 複製先を決める(複製元: &複製元リポジトリURL, 複製先の指定: Option<複製先ディレクトリ>) -> Result<複製先ディレクトリ, 複製エラー> {
    match 複製先の指定 {
        Some(先) => Ok(先),
        None => 複製先ディレクトリ::複製元から導く(複製元)
            .ok_or_else(|| 複製エラー::複製先ディレクトリ名を導けない { 綴り: 複製元.文字列表現().to_owned() }),
    }
}

fn 複製した作業ツリーを使える状態にする(操作: &対象リポジトリのGitLFS操作) -> Result<(), 複製エラー> {
    操作.フィルターをこのリポジトリだけへ登録する()?;
    let 実行場所 = Gitコマンドの実行場所::指定ディレクトリから生成する(操作.作業ツリーのパス());
    install_command::対象リポジトリへ設定を登録する(実行場所, None)?;
    論理プロファイルが解決できることを確かめる(操作)?;
    操作.実体を取得する()
}

/// 実体の取得の前に、このPCのPC設定が作業ツリーの指す論理プロファイルを持つことを確かめる
/// （判断3）。
fn 論理プロファイルが解決できることを確かめる(操作: &対象リポジトリのGitLFS操作) -> Result<(), 複製エラー> {
    let 場所 = pc設定の場所を解決する().map_err(|エラー| 複製エラー::論理プロファイルの解決に失敗 { 説明: エラー.to_string() })?;
    let 置き場所 = プロファイル解決に使う設定の置き場所::生成する(操作.作業ツリーのパス(), 場所);
    置き場所
        .論理プロファイルを解決する()
        .map(|_| ())
        .map_err(|エラー| 複製エラー::論理プロファイルの解決に失敗 { 説明: エラー.to_string() })
}

fn 複製した作業ツリーの場所と続きの手順を知らせる(複製先: &複製先ディレクトリ) {
    eprintln!("複製した作業ツリーは消さずに残しました: {}", 複製先.表示用の綴り());
    eprintln!("原因を取り除いたあと、その作業ツリーで次を実行すれば続きから再開できます。");
    eprintln!("  git-lfs-rclone-storage doctor");
    eprintln!("  git lfs pull");
}
