//! `check-objects`サブコマンド: Git LFSが参照するオブジェクトが保管先に実在するかを
//! 突き合わせ、欠けたものだけを一覧で示すこと、保管先に在ってGit LFSが参照しない
//! オブジェクト（他のリポジトリが置いたもの）を違反として報告しないことを確かめる。
//! ローカルディレクトリ方式で行うためrcloneに依存せず、既定で実行する。

mod common;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const プロファイル名: &str = "check-objects";

struct 点検の舞台 {
    保管先ルート: tempfile::TempDir,
    一時ディレクトリ: tempfile::TempDir,
    作業ツリー: tempfile::TempDir,
    pc設定: tempfile::TempDir,
}

fn 舞台を作る() -> Result<点検の舞台, Box<dyn std::error::Error>> {
    let 保管先ルート = tempfile::tempdir()?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る(プロファイル名)?;
    common::git_fixture::lfsを有効化する(作業ツリー.path())?;
    common::git_fixture::追跡パターンを登録する(作業ツリー.path(), "*.bin")?;
    let pc設定 = common::fixtures::ローカル方式のpc設定ディレクトリを作る(プロファイル名, 保管先ルート.path(), 一時ディレクトリ.path())?;
    Ok(点検の舞台 { 保管先ルート, 一時ディレクトリ, 作業ツリー, pc設定 })
}

fn 識別子を求める(内容: &[u8]) -> String {
    Sha256::digest(内容).iter().map(|バイト| format!("{バイト:02x}")).collect()
}

fn 保管先のオブジェクトパス(保管先ルート: &Path, 識別子: &str) -> PathBuf {
    保管先ルート.join("lfs").join("objects").join("sha256").join(&識別子[0..2]).join(&識別子[2..4]).join(識別子)
}

/// agentがアップロードを終えた状態を、保管先へ実体を直接置くことで作る。
fn 保管先へ実体を置く(保管先ルート: &Path, 内容: &[u8]) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let パス = 保管先のオブジェクトパス(保管先ルート, &識別子を求める(内容));
    std::fs::create_dir_all(パス.parent().ok_or("親ディレクトリがありません")?)?;
    std::fs::write(&パス, 内容)?;
    Ok(パス)
}

fn 点検を実行する(
    舞台: &点検の舞台,
    引数: &[&str],
) -> Result<common::cli_invocation::サブコマンド実行結果, Box<dyn std::error::Error>> {
    common::cli_invocation::サブコマンドを実行する(
        舞台.作業ツリー.path(),
        引数,
        &[("LFS_RCLONE_PC_CONFIG_DIR", OsStr::new(舞台.pc設定.path()))],
    )
}

#[test]
fn 全て保管先に在れば成功し他のリポジトリのオブジェクトを違反にしない() -> Result<(), Box<dyn std::error::Error>> {
    let 舞台 = 舞台を作る()?;
    let 内容: &[u8] = b"check-objects present payload";
    common::git_fixture::ファイルを追加してコミットする(舞台.作業ツリー.path(), "asset.bin", 内容)?;
    保管先へ実体を置く(舞台.保管先ルート.path(), 内容)?;
    保管先へ実体を置く(舞台.保管先ルート.path(), b"payload that belongs to another repository")?;

    let 結果 = 点検を実行する(&舞台, &["check-objects"])?;

    assert!(結果.成功したか, "全て在れば0終了するべき: stdout={} stderr={}", 結果.標準出力, 結果.標準エラー出力);
    assert!(結果.標準出力.contains("点検したオブジェクト: 1件"), "点検した件数を出すべき: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("保管先に見つからないオブジェクト: 0件"), "欠落0件を出すべき: {}", 結果.標準出力);
    assert!(!結果.標準出力.contains("[欠落]"), "他のリポジトリのオブジェクトを違反として報告してはならない: {}", 結果.標準出力);
    let _ = &舞台.一時ディレクトリ;
    Ok(())
}

#[test]
fn 保管先から実体が消えていれば欠落として一覧に出す() -> Result<(), Box<dyn std::error::Error>> {
    let 舞台 = 舞台を作る()?;
    let 残す内容: &[u8] = b"check-objects surviving payload";
    let 消す内容: &[u8] = b"check-objects deleted payload";
    common::git_fixture::ファイルを追加してコミットする(舞台.作業ツリー.path(), "kept.bin", 残す内容)?;
    common::git_fixture::ファイルを追加してコミットする(舞台.作業ツリー.path(), "lost.bin", 消す内容)?;
    保管先へ実体を置く(舞台.保管先ルート.path(), 残す内容)?;
    let 消す実体 = 保管先へ実体を置く(舞台.保管先ルート.path(), 消す内容)?;
    std::fs::remove_file(&消す実体)?;

    let 結果 = 点検を実行する(&舞台, &["check-objects"])?;

    assert!(!結果.成功したか, "欠落があれば非0終了するべき: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("保管先に見つからないオブジェクト: 1件"), "欠落件数を出すべき: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("[欠落] lost.bin"), "欠落したファイルのパスを出すべき: {}", 結果.標準出力);
    assert!(結果.標準出力.contains(&識別子を求める(消す内容)), "欠落したオブジェクトの識別子を出すべき: {}", 結果.標準出力);
    assert!(!結果.標準出力.contains("kept.bin"), "保管先に在るファイルを欠落として出してはならない: {}", 結果.標準出力);
    assert!(結果.標準出力.contains("git lfs push --all origin"), "送り直す対処を出すべき: {}", 結果.標準出力);
    Ok(())
}

#[test]
fn 全履歴を指定すると過去の版の実体も点検する() -> Result<(), Box<dyn std::error::Error>> {
    let 舞台 = 舞台を作る()?;
    let 現在の内容: &[u8] = b"check-objects current payload";
    let 過去の内容: &[u8] = b"check-objects past payload";
    common::git_fixture::ファイルを追加してコミットする(舞台.作業ツリー.path(), "past.bin", 過去の内容)?;
    common::git_fixture::ファイルを削除してコミットする(舞台.作業ツリー.path(), "past.bin")?;
    common::git_fixture::ファイルを追加してコミットする(舞台.作業ツリー.path(), "current.bin", 現在の内容)?;
    保管先へ実体を置く(舞台.保管先ルート.path(), 現在の内容)?;

    let 既定の結果 = 点検を実行する(&舞台, &["check-objects"])?;
    assert!(既定の結果.成功したか, "現在のチェックアウトだけなら成功するべき: {}", 既定の結果.標準出力);
    assert!(既定の結果.標準出力.contains("点検の範囲: 現在のチェックアウト"), "範囲を出すべき: {}", 既定の結果.標準出力);

    let 全履歴の結果 = 点検を実行する(&舞台, &["check-objects", "--all"])?;
    assert!(!全履歴の結果.成功したか, "過去の版の実体が無ければ非0終了するべき: {}", 全履歴の結果.標準出力);
    assert!(全履歴の結果.標準出力.contains("点検の範囲: 全履歴"), "範囲を出すべき: {}", 全履歴の結果.標準出力);
    assert!(全履歴の結果.標準出力.contains("[欠落] past.bin"), "過去の版のファイルを欠落として出すべき: {}", 全履歴の結果.標準出力);
    Ok(())
}
