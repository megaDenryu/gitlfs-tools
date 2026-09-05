//! rcloneプロセスの起動・実行に関する失敗の分類。起動失敗・非0終了・タイムアウトの
//! 3種を区別する（Issue #5「rclone起動不能、非0終了、timeoutを分類する」）。
//!
//! 注意: 標準エラー出力の内容はどの分類にも保持しない。remoteの秘密情報が診断メッセージへ
//! 混入する経路を型の設計自体で断つため、表示文には操作名と終了コードだけを載せる
//! （アーキテクチャ.md「認証情報を混入させない」）。

use gitlfs_tools_domain::保管エラー;

use crate::rclone_operation::Rclone操作;

#[derive(Debug)]
pub(crate) enum Rclone実行エラー {
    起動失敗 { 操作: Rclone操作, 説明: String },
    非0終了 { 操作: Rclone操作, 終了コード: Option<i32> },
    タイムアウト { 操作: Rclone操作 },
}

impl std::fmt::Display for Rclone実行エラー {
    fn fmt(&self, フォーマッタ: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::起動失敗 { 操作, 説明 } => {
                write!(フォーマッタ, "rcloneの{}の起動に失敗しました: {説明}", 操作.名称())
            }
            Self::非0終了 { 操作, 終了コード: Some(コード) } => {
                write!(フォーマッタ, "rcloneの{}が終了コード{コード}で失敗しました", 操作.名称())
            }
            Self::非0終了 { 操作, 終了コード: None } => {
                write!(フォーマッタ, "rcloneの{}が終了コード不明のまま異常終了しました", 操作.名称())
            }
            Self::タイムアウト { 操作 } => {
                write!(フォーマッタ, "rcloneの{}がタイムアウトしました", 操作.名称())
            }
        }
    }
}

impl From<Rclone実行エラー> for 保管エラー {
    fn from(エラー: Rclone実行エラー) -> Self {
        保管エラー::子プロセス { 説明: エラー.to_string() }
    }
}
