//! リポジトリ配下のRustファイルを実際に読み、ファイルごとのコード行数を計測する。

use crate::line_count::code_line_count::コード行数;
use crate::line_count::relative_rust_file_path::相対rustファイルパス;
use crate::line_count::repository_scan::リポジトリルート;
use crate::line_count::rust_source::Rustソース;

#[derive(Clone)]
pub struct 実測行数 {
    pub ファイル: 相対rustファイルパス,
    pub 行数: コード行数,
}

impl リポジトリルート {
    pub fn コード行数を計測する(&self) -> Result<Vec<実測行数>, String> {
        let ファイル一覧 = self.rustファイル一覧を列挙する()?;
        let mut 結果 = Vec::with_capacity(ファイル一覧.len());

        for ファイル in ファイル一覧 {
            let 絶対パス = self.絶対パスに変換する(&ファイル);
            let 内容 = std::fs::read_to_string(&絶対パス)
                .map_err(|失敗| format!("{}を読み込めなかった: {失敗}", 絶対パス.display()))?;
            let 行数 = Rustソース::生成する(内容).コード行数を数える();
            結果.push(実測行数 { ファイル, 行数 });
        }

        Ok(結果)
    }
}
