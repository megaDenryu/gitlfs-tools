//! ローカルの一時保存先を表す値型。

use std::path::PathBuf;

use crate::temp_file_path::一時ファイルパス;
use crate::unique_identifier::一意な識別子;

/// ローカルの一時保存先。存在確認や作成はこの型の責務ではない
/// （domain層はファイルシステムの状態を知らない。作成・検証は上位層が行う）。
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct 一時ディレクトリ(PathBuf);

impl 一時ディレクトリ {
    pub fn 生成する(ルート: impl Into<PathBuf>) -> Self {
        Self(ルート.into())
    }

    /// 呼ぶたびに異なる、衝突しない一時ファイルパスを払い出す。
    pub fn 固有の一時ファイルパスを払い出す(&self) -> 一時ファイルパス {
        let 識別子 = 一意な識別子::発行する();
        一時ファイルパス::生成する(self.0.join(識別子.文字列表現()))
    }
}
