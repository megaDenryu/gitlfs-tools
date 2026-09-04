//! 利用者向けのエラーコード対応表が、実装の番号割り当てと一致していることを検査する
//! コマンド。
//!
//! 対応表は`_doc/利用/トラブルシューティング.md`にあり、利用者はエラー出力の角括弧内の
//! 数字をこの表で引く。表が実装から取り残されると、利用者は存在しない説明を読むことに
//! なる。文書の正しさも規約と同じく機械で強制するため、`cargo xtask verify`の工程へ
//! 加えてある。

mod document_table;
mod error_code_number;
mod implementation_source;
mod violation;

use std::path::PathBuf;

use crate::command_registry::サブコマンド;

pub struct エラーコード対応表検査コマンド;

impl サブコマンド for エラーコード対応表検査コマンド {
    fn 名前(&self) -> &'static str {
        "check-error-code-table"
    }

    fn 説明(&self) -> &'static str {
        "トラブルシューティング.mdのエラーコード対応表と実装の番号割り当ての一致を検査する"
    }

    fn 実行する(&self, _引数: &[String]) -> Result<(), String> {
        検査対象の配置::カレントディレクトリから生成する()?.突き合わせる()
    }
}

struct 検査対象の配置 {
    実装ソース: PathBuf,
    対応表文書: PathBuf,
}

impl 検査対象の配置 {
    fn カレントディレクトリから生成する() -> Result<Self, String> {
        let ルート = std::env::current_dir()
            .map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))?;
        Ok(Self {
            実装ソース: ルート.join("crates").join("lfs-rclone-protocol").join("src").join("error_code.rs"),
            対応表文書: ルート.join("_doc").join("利用").join("トラブルシューティング.md"),
        })
    }

    fn 突き合わせる(&self) -> Result<(), String> {
        let 実装一覧 = implementation_source::実装の一覧を読み取る(&self.実装ソース)?;
        let 文書一覧 = document_table::文書の一覧を読み取る(&self.対応表文書)?;
        let 違反一覧 = violation::違反を判定する(&実装一覧, &文書一覧);

        violation::結果を報告する(実装一覧.len(), 文書一覧.len(), &違反一覧)
    }
}
