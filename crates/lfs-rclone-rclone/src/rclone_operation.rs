//! rcloneへ委ねる操作の種別。診断メッセージでの識別にだけ使う内部区分であり、
//! ドメインの状態遷移は表さない。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rclone操作 {
    存在確認,
    オブジェクト数の集計,
    アップロード転送,
    最終化転送,
    ダウンロード転送,
}

impl Rclone操作 {
    pub(crate) fn 名称(self) -> &'static str {
        match self {
            Self::存在確認 => "存在確認(lsjson)",
            Self::オブジェクト数の集計 => "オブジェクト数の集計(size)",
            Self::アップロード転送 => "アップロード転送(copyto)",
            Self::最終化転送 => "最終化転送(moveto)",
            Self::ダウンロード転送 => "ダウンロード転送(copyto)",
        }
    }
}
