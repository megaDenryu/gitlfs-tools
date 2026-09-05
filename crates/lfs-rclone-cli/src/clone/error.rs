//! `clone`サブコマンドが実行時に返す失敗の分類。
//! 「エラー分類」役割のファイルである（コード分割規約.md 1節）。

use crate::command_error::コマンド実行エラー;

#[derive(Debug, thiserror::Error)]
pub(crate) enum 複製エラー {
    #[error("複製先のディレクトリ名を'{綴り}'から導けませんでした。第2引数でディレクトリ名を指定してください")]
    複製先ディレクトリ名を導けない { 綴り: String },

    #[error("gitコマンドを起動できませんでした: {説明}")]
    Gitコマンド起動失敗 { 説明: String },

    #[error("git cloneに失敗しました: {説明}")]
    複製に失敗 { 説明: String },

    #[error("git lfs install --localに失敗しました: {説明}")]
    フィルター登録に失敗 { 説明: String },

    #[error("custom transfer agentの登録に失敗しました: {説明}")]
    転送agentの登録に失敗 { 説明: String },

    #[error("このリポジトリが指す論理プロファイルを解決できませんでした: {説明}")]
    論理プロファイルの解決に失敗 { 説明: String },

    #[error("git lfs pullに失敗しました: {説明}")]
    実体の取得に失敗 { 説明: String },
}

impl 複製エラー {
    /// 失敗の説明に続けて、利用者が次に見る場所を示す。論理プロファイルの不足は、他人の
    /// リポジトリを初めて取るときに必ず起きる詰まり方であるため、案内を欠かさない。
    pub(crate) fn 続けて示す案内を表示する(&self) {
        if let Self::論理プロファイルの解決に失敗 { .. } = self {
            eprintln!("PC設定の書き方は_doc/利用/PC初期設定.mdにあります。");
            eprintln!("複製した作業ツリーでdoctorを実行すると、不足の全体が分かります。");
        }
    }
}

impl From<コマンド実行エラー> for 複製エラー {
    fn from(エラー: コマンド実行エラー) -> Self {
        Self::転送agentの登録に失敗 { 説明: エラー.to_string() }
    }
}
