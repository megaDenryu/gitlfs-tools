//! サブコマンド名・引数の解釈で起こりうる失敗の分類。

#[derive(Debug, thiserror::Error)]
pub(crate) enum 起動引数エラー {
    #[error("未知のサブコマンドです: {名前}")]
    未知のサブコマンド { 名前: String },

    #[error("{サブコマンド}に未知の引数が渡されました: {名前}")]
    未知の引数 { サブコマンド: String, 名前: String },

    #[error("{サブコマンド}の引数{引数名}には値が必要です")]
    値が必要な引数 { サブコマンド: String, 引数名: String },

    #[error("cloneには複製元のURLが必要です")]
    複製元のURLが必要,

    #[error("init-projectには--profileが必要です")]
    プロファイル名が必要,

    #[error("プロファイル名が不正です: {説明}")]
    プロファイル名が不正 { 説明: String },
}
