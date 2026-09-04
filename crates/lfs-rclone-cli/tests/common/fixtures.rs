//! 結合テスト用の設定ファイル一式（PC設定・プロジェクト設定）を隔離した一時ディレクトリへ
//! 書き出すヘルパー。実ユーザーの設定ディレクトリへは一切触れない
//! （CLAUDE.md「実ユーザーの設定ディレクトリを読み書きするテストを書いてはならない」）。

use std::fs;
use std::path::Path;

use tempfile::TempDir;

pub const 実行ファイルのパス: &str = env!("CARGO_BIN_EXE_git-lfs-rclone-storage");

/// 絶対パスをドライブ文字とrclone local backend用の残りパスへ分ける
/// （`lfs-rclone-rclone`のlocal backend結合テストと同じ手法）。
pub fn ドライブとパスへ分ける(絶対パス: &Path) -> Result<(String, String), Box<dyn std::error::Error>> {
    let 文字列 = 絶対パス.to_str().ok_or("パスがUTF-8ではありません")?;
    let 正規化 = 文字列.replace('\\', "/");
    let (ドライブ, 残り) = 正規化.split_once(':').ok_or("絶対パスにドライブ文字がありません")?;
    Ok((ドライブ.to_owned(), 残り.to_owned()))
}

/// `.large-assets.toml`だけを含むプロジェクト作業ツリーを、隔離したGitリポジトリとして作る。
/// agentはダウンロードの一時ファイル置き場をGitリポジトリから決めるため、`init`を通す
/// テストの作業ツリーは実際のリポジトリでなければならない（Git LFSが起動する実環境と同じ）。
pub fn プロジェクト作業ツリーを作る(プロファイル名: &str) -> Result<TempDir, Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 内容 = format!("schema_version = 1\nprofile = \"{プロファイル名}\"\n");
    fs::write(ディレクトリ.path().join(".large-assets.toml"), 内容)?;
    super::git_fixture::初期化する(ディレクトリ.path())?;
    Ok(ディレクトリ)
}

/// このプロジェクト作業ツリーでagentがダウンロードの一時ファイルを置く場所。
pub fn ダウンロード一時ディレクトリのパス(作業ツリー: &Path) -> std::path::PathBuf {
    作業ツリー.join(".git").join("lfs").join("tmp").join("rclone-storage-agent")
}

/// 指定した1プロファイルだけを持つPC設定ディレクトリを作る。`rclone_executable`は省略時
/// `None`でPATH解決に委ねる。
pub fn pc設定ディレクトリを作る(
    プロファイル名: &str,
    リモート: &str,
    基底パス: &str,
    一時ディレクトリ: &Path,
    rclone実行ファイル: Option<&Path>,
) -> Result<TempDir, Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 一時ディレクトリ文字列 = 一時ディレクトリ.to_string_lossy().replace('\\', "/");
    let rclone_executable行 = rclone実行ファイル
        .map(|パス| format!("rclone_executable = \"{}\"\n", パス.to_string_lossy().replace('\\', "/")))
        .unwrap_or_default();
    let 内容 = format!(
        "schema_version = 1\n\
         [profiles.{プロファイル名}]\n\
         rclone_remote = \"{リモート}\"\n\
         base_path = \"{基底パス}\"\n\
         temp_directory = \"{一時ディレクトリ文字列}\"\n\
         transfer_timeout_seconds = 30\n\
         {rclone_executable行}"
    );
    fs::write(ディレクトリ.path().join("config.toml"), 内容)?;
    Ok(ディレクトリ)
}

/// `storage = "local"`の1プロファイルだけを持つPC設定ディレクトリを作る。ローカル
/// ディレクトリ方式ではrcloneの3キーを書けないため、この関数は受け取らない。
pub fn ローカル方式のpc設定ディレクトリを作る(
    プロファイル名: &str,
    保管先ルート: &Path,
    一時ディレクトリ: &Path,
) -> Result<TempDir, Box<dyn std::error::Error>> {
    let ディレクトリ = tempfile::tempdir()?;
    let 内容 = format!(
        "schema_version = 1\n\
         [profiles.{プロファイル名}]\n\
         storage = \"local\"\n\
         base_path = \"{}\"\n\
         temp_directory = \"{}\"\n",
        保管先ルート.to_string_lossy().replace('\\', "/"),
        一時ディレクトリ.to_string_lossy().replace('\\', "/")
    );
    fs::write(ディレクトリ.path().join("config.toml"), 内容)?;
    Ok(ディレクトリ)
}
