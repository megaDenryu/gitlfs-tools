//! `check-objects`が点検する範囲を表す判別共用体。範囲の違いは`git lfs ls-files`へ渡す
//! 引数の違いとして現れるため、引数の綴りをこの型のメソッドへ閉じる。

/// 点検の範囲。既定は現在のチェックアウトであり、`--all`で全履歴まで広げる。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 点検範囲 {
    現在のチェックアウト,
    全履歴,
}

impl 点検範囲 {
    /// `git lfs ls-files`へ渡す引数。`--json`は正確なバイト数を得るために常に付ける
    /// （`-s`が出すサイズは人向けに丸めた表示であり、突き合わせに使えない）。
    pub(crate) fn ls_files引数(self) -> &'static [&'static str] {
        match self {
            Self::現在のチェックアウト => &["lfs", "ls-files", "--json"],
            Self::全履歴 => &["lfs", "ls-files", "--json", "--all"],
        }
    }

    pub(crate) fn 説明(self) -> &'static str {
        match self {
            Self::現在のチェックアウト => "現在のチェックアウト",
            Self::全履歴 => "全履歴(過去の版を含む)",
        }
    }
}
