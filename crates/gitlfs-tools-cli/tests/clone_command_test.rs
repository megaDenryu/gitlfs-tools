//! `clone`サブコマンド: 実git・実git-lfsと、ローカルディレクトリ方式の保管先を使って、
//! 送信済みの実体を1コマンドで取り戻せることを確かめる。rcloneを使わないため、この結合
//! テストは環境によらず既定で実行する（読み飛ばしの経路を持たない）。
//!
//! 実ユーザーの設定には触れない。PC設定は`GITLFS_TOOLS_PC_CONFIG_DIR`で、Gitのグローバル
//! 設定は`GIT_CONFIG_GLOBAL`で、どちらも隔離した一時ディレクトリへ差し替える。

mod common;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

const プロファイル名: &str = "clone-roundtrip";
const 実体の内容: &[u8] = b"clone roundtrip payload";

#[test]
fn cloneは実体を含む作業ツリーを作る() -> Result<(), Box<dyn std::error::Error>> {
    let 環境 = 送信済みの試験環境::用意する()?;
    let 複製先の親 = tempfile::tempdir()?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(
        複製先の親.path(),
        &["clone", &環境.originの綴り()],
        &環境.環境変数を組む(&環境.pc設定ディレクトリ),
    )?;

    assert!(結果.成功したか, "cloneは成功するべき: stderr={}", 結果.標準エラー出力);
    let 取得したファイル = 複製先の親.path().join("origin").join("payload.bin");
    assert_eq!(std::fs::read(&取得したファイル)?, 実体の内容, "pointerが実体へ置き換わっているべき");
    Ok(())
}

#[test]
fn 論理プロファイルを解決できないとき作業ツリーを残して止まる() -> Result<(), Box<dyn std::error::Error>> {
    let 環境 = 送信済みの試験環境::用意する()?;
    let 別の保管先 = tempfile::tempdir()?;
    let 別の一時ディレクトリ = tempfile::tempdir()?;
    let 別のpc設定 =
        common::fixtures::ローカル方式のpc設定ディレクトリを作る("another-profile", 別の保管先.path(), 別の一時ディレクトリ.path())?;
    let 複製先の親 = tempfile::tempdir()?;

    let 結果 = common::cli_invocation::サブコマンドを実行する(
        複製先の親.path(),
        &["clone", &環境.originの綴り()],
        &環境.環境変数を組む(別のpc設定.path()),
    )?;

    assert!(!結果.成功したか, "解決できない論理プロファイルのままcloneが成功してはならない");
    assert!(結果.標準エラー出力.contains(プロファイル名), "不足している論理プロファイル名を示すべき: {}", 結果.標準エラー出力);
    let 複製先 = 複製先の親.path().join("origin");
    assert!(複製先.join(".git").is_dir(), "複製した作業ツリーを消してはならない");
    assert_ne!(std::fs::read(複製先.join("payload.bin"))?, 実体の内容, "実体の取得は行われていないべき");
    Ok(())
}

/// 実体を送信済みのoriginと、その実体を持つ保管先を用意した状態。一時ディレクトリは
/// 破棄されると中身が消えるため、試験の間だけ生かしておく必要がある。
struct 送信済みの試験環境 {
    pc設定ディレクトリ: PathBuf,
    グローバル設定ファイル: PathBuf,
    origin: PathBuf,
    _寿命を保つ一時ディレクトリ一覧: Vec<TempDir>,
}

impl 送信済みの試験環境 {
    fn 用意する() -> Result<Self, Box<dyn std::error::Error>> {
        let 保管先ルート = tempfile::tempdir()?;
        let 一時ディレクトリ = tempfile::tempdir()?;
        let pc設定 =
            common::fixtures::ローカル方式のpc設定ディレクトリを作る(プロファイル名, 保管先ルート.path(), 一時ディレクトリ.path())?;
        let グローバル設定の置き場 = tempfile::tempdir()?;
        let グローバル設定ファイル = グローバル設定の置き場.path().join("gitconfig");
        std::fs::write(&グローバル設定ファイル, "")?;
        let originの親 = tempfile::tempdir()?;
        let origin = originの親.path().join("origin.git");
        common::git_fixture::裸リポジトリを初期化する(&origin)?;
        let 送信元 = tempfile::tempdir()?;

        let 環境変数 = [
            ("GITLFS_TOOLS_PC_CONFIG_DIR", pc設定.path().as_os_str()),
            ("GIT_CONFIG_GLOBAL", グローバル設定ファイル.as_os_str()),
        ];
        送信元を仕立てて実体を送信する(送信元.path(), &origin, &環境変数)?;

        Ok(Self {
            pc設定ディレクトリ: pc設定.path().to_owned(),
            グローバル設定ファイル,
            origin,
            _寿命を保つ一時ディレクトリ一覧: vec![保管先ルート, 一時ディレクトリ, pc設定, グローバル設定の置き場, originの親, 送信元],
        })
    }

    /// `clone`へ渡す複製元の綴り。末尾が`origin.git`であるため、複製先の名前は`origin`へ導かれる。
    fn originの綴り(&self) -> String {
        self.origin.to_string_lossy().into_owned()
    }

    fn 環境変数を組む<'置き場>(&'置き場 self, pc設定ディレクトリ: &'置き場 Path) -> [(&'static str, &'置き場 OsStr); 2] {
        [
            ("GITLFS_TOOLS_PC_CONFIG_DIR", pc設定ディレクトリ.as_os_str()),
            ("GIT_CONFIG_GLOBAL", self.グローバル設定ファイル.as_os_str()),
        ]
    }
}

fn 送信元を仕立てて実体を送信する(
    送信元: &Path,
    origin: &Path,
    環境変数: &[(&str, &OsStr)],
) -> Result<(), Box<dyn std::error::Error>> {
    common::git_fixture::初期化する(送信元)?;
    std::fs::write(送信元.join(".large-assets.toml"), format!("schema_version = 1\nprofile = \"{プロファイル名}\"\n"))?;
    common::git_fixture::lfsを有効化する(送信元)?;
    common::git_fixture::追跡パターンを登録する(送信元, "*.bin")?;
    let 導入結果 = common::cli_invocation::サブコマンドを実行する(送信元, &["install"], 環境変数)?;
    assert!(導入結果.成功したか, "送信元へのinstallは成功するべき: stderr={}", 導入結果.標準エラー出力);
    common::git_fixture::ファイルを追加してコミットする(送信元, "payload.bin", 実体の内容)?;
    common::git_fixture::環境変数を与えて実行する(送信元, &["remote", "add", "origin", &origin.to_string_lossy()], 環境変数)?;
    common::git_fixture::環境変数を与えて実行する(送信元, &["push", "-q", "origin", "HEAD"], 環境変数)
}
