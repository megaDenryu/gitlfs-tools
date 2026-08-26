//! `_doc/設計/行数の例外台帳.md`の「## 登録」表を読み取る。
//!
//! 表の書式は`| ファイル | 上限 | 統合した理由 |`である。ファイル欄が`.rs`で終わらない
//! 行（見出し・区切り線・「（現在なし）」の空行）は登録として扱わない。

use std::path::Path;

use crate::line_count::line_count_limit::行数上限;
use crate::line_count::relative_rust_file_path::相対rustファイルパス;

pub struct 台帳登録 {
    pub ファイル: 相対rustファイルパス,
    pub 上限: 行数上限,
}

pub struct 台帳 {
    pub 登録一覧: Vec<台帳登録>,
}

impl 台帳 {
    pub fn ファイルから読み込む(台帳ファイル: &Path) -> Result<Self, String> {
        let 内容 = std::fs::read_to_string(台帳ファイル)
            .map_err(|失敗| format!("{}を読み込めなかった: {失敗}", 台帳ファイル.display()))?;

        let mut 登録一覧 = Vec::new();
        for 行 in 内容.lines() {
            let 行 = 行.trim();
            if !行.starts_with('|') {
                continue;
            }

            let セル: Vec<&str> = 行.trim_matches('|').split('|').map(str::trim).collect();
            let [ファイル欄, 上限欄, ..] = セル.as_slice() else {
                continue;
            };

            if !ファイル欄.ends_with(".rs") {
                continue;
            }

            let 上限値 = 上限欄.parse::<usize>().map_err(|失敗| {
                format!("台帳の{ファイル欄}の上限「{上限欄}」を数値として読み取れなかった: {失敗}")
            })?;

            登録一覧.push(台帳登録 {
                ファイル: 相対rustファイルパス::生成する((*ファイル欄).to_owned()),
                上限: 行数上限::生成する(上限値),
            });
        }

        Ok(Self { 登録一覧 })
    }
}
