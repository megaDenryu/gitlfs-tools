//! リポジトリルートの配置と、その配下にある`.rs`ファイルの列挙。
//!
//! 走査対象は`crates/`と`xtask/`の2つの部分木であり、`target/`は走査しない。

use std::path::{Path, PathBuf};

use crate::line_count::relative_rust_file_path::相対rustファイルパス;

pub struct リポジトリルート(PathBuf);

impl リポジトリルート {
    pub fn カレントディレクトリから生成する() -> Result<Self, String> {
        std::env::current_dir()
            .map(Self)
            .map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))
    }

    pub fn 台帳のパス(&self) -> PathBuf {
        self.0.join("_doc").join("設計").join("行数の例外台帳.md")
    }

    pub fn 絶対パスに変換する(&self, 相対: &相対rustファイルパス) -> PathBuf {
        self.0.join(相対.文字列表現())
    }

    pub fn rustファイル一覧を列挙する(&self) -> Result<Vec<相対rustファイルパス>, String> {
        let mut 一覧 = Vec::new();
        for 走査対象名 in ["crates", "xtask"] {
            let 走査対象 = self.0.join(走査対象名);
            if 走査対象.is_dir() {
                走査対象を再帰走査してrustファイルを集める(&走査対象, &self.0, &mut 一覧)?;
            }
        }
        一覧.sort();
        Ok(一覧)
    }
}

fn 走査対象を再帰走査してrustファイルを集める(
    対象ディレクトリ: &Path,
    ルートパス: &Path,
    収集先: &mut Vec<相対rustファイルパス>,
) -> Result<(), String> {
    let 読み取り結果 = std::fs::read_dir(対象ディレクトリ)
        .map_err(|失敗| format!("{}を読み取れなかった: {失敗}", 対象ディレクトリ.display()))?;

    for 項目 in 読み取り結果 {
        let 項目 = 項目.map_err(|失敗| format!("ディレクトリ項目を読み取れなかった: {失敗}"))?;
        let パス = 項目.path();

        if パス.is_dir() {
            if パス.file_name().and_then(|名前| 名前.to_str()) == Some("target") {
                continue;
            }
            走査対象を再帰走査してrustファイルを集める(&パス, ルートパス, 収集先)?;
        } else if パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("rs") {
            収集先.push(絶対パスを相対rustファイルパスへ変換する(&パス, ルートパス)?);
        }
    }
    Ok(())
}

fn 絶対パスを相対rustファイルパスへ変換する(
    絶対パス: &Path,
    ルートパス: &Path,
) -> Result<相対rustファイルパス, String> {
    let 相対 = 絶対パス
        .strip_prefix(ルートパス)
        .map_err(|失敗| format!("{}を相対パスに変換できなかった: {失敗}", 絶対パス.display()))?;
    let 正規化済み文字列 = 相対
        .components()
        .map(|部品| 部品.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/");
    Ok(相対rustファイルパス::生成する(正規化済み文字列))
}
