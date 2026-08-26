//! Git LFS protocolの応答（確認応答・進捗・完了・初期化失敗）を組み立てて標準出力へ送る
//! 外部境界。JSONの形をここへ閉じ、`protocol_session.rs`はドメインの値だけを渡す。

use std::io;

use crate::outgoing_event_json::{エラーJSON, 完了JSON, 初期化失敗JSON, 確認応答JSON, 進捗JSON};
use crate::presentable_error::表示用エラー;
use crate::stdout_writer::標準出力書き込み器;

pub(crate) struct プロトコル応答送信器 {
    書き込み器: 標準出力書き込み器,
}

impl プロトコル応答送信器 {
    pub(crate) fn 生成する() -> Self {
        Self { 書き込み器: 標準出力書き込み器::生成する() }
    }

    pub(crate) fn 初期化成功を送る(&self) -> io::Result<()> {
        self.書き込み器.一行書く(&確認応答JSON {})
    }

    pub(crate) fn 初期化失敗を送る(&self, エラー: &表示用エラー) -> io::Result<()> {
        self.書き込み器.一行書く(&初期化失敗JSON { error: エラー本体を組み立てる(エラー) })
    }

    pub(crate) fn 進捗を送る(&self, oid: &str, size: u64) -> io::Result<()> {
        self.書き込み器.一行書く(&進捗JSON {
            event: "progress",
            oid: oid.to_owned(),
            bytes_so_far: size,
            bytes_since_last: size,
        })
    }

    pub(crate) fn アップロード完了を送る(&self, oid: &str) -> io::Result<()> {
        self.書き込み器.一行書く(&完了JSON { event: "complete", oid: oid.to_owned(), path: None, error: None })
    }

    pub(crate) fn ダウンロード完了を送る(&self, oid: &str, path: &str) -> io::Result<()> {
        self.書き込み器.一行書く(&完了JSON {
            event: "complete",
            oid: oid.to_owned(),
            path: Some(path.to_owned()),
            error: None,
        })
    }

    pub(crate) fn 失敗完了を送る(&self, oid: &str, エラー: &表示用エラー) -> io::Result<()> {
        self.書き込み器.一行書く(&完了JSON {
            event: "complete",
            oid: oid.to_owned(),
            path: None,
            error: Some(エラー本体を組み立てる(エラー)),
        })
    }
}

fn エラー本体を組み立てる(エラー: &表示用エラー) -> エラーJSON {
    エラーJSON { code: エラー.コード().値(), message: エラー.メッセージ().to_owned() }
}
