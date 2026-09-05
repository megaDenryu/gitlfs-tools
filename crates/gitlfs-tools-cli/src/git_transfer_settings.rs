//! `install`が対象リポジトリのGit設定へ書き込む4つのキーと値。キーの綴りはGit LFS
//! 公式仕様が固定する（CLAUDE.md「命名」）。根拠は`lfs-custom-transfer-protocol.md`8節
//! （standalone transfer agentの設定キー）。カスタム転送名は、この基盤の名前と揃えて
//! `gitlfs-tools`とする（Issue #2 3節がカスタム転送名の指定を求めている）。
//! `lfs.standalonetransferagent`へ同じ名前を指定してこのリポジトリだけを
//! standalone agent経由にする。`--local`スコープで書くため他のリポジトリへは波及しない。

use crate::install_target_path::登録する実行ファイルパス;

const カスタム転送名: &str = "gitlfs-tools";

pub(crate) const 設定キー一覧: [&str; 4] = [
    "lfs.customtransfer.gitlfs-tools.path",
    "lfs.customtransfer.gitlfs-tools.concurrent",
    "lfs.customtransfer.gitlfs-tools.direction",
    "lfs.standalonetransferagent",
];

pub(crate) struct Git転送設定 {
    実行ファイルパス: 登録する実行ファイルパス,
}

impl Git転送設定 {
    pub(crate) fn 生成する(実行ファイルパス: 登録する実行ファイルパス) -> Self {
        Self { 実行ファイルパス }
    }

    /// `concurrent`はgit-lfs自身の既定値でもある`true`を明示する。各プロセスは固有の
    /// UUID一時パスへ書き込んでから最終パスへ移すため（アーキテクチャ.md 判断5）、
    /// 並行起動しても内容は競合しない。`direction`はIssue #2 7.3節の指示どおり`both`。
    pub(crate) fn キーと値の一覧(&self) -> [(&'static str, String); 4] {
        [
            (設定キー一覧[0], self.実行ファイルパス.設定値の文字列表現()),
            (設定キー一覧[1], "true".to_owned()),
            (設定キー一覧[2], "both".to_owned()),
            (設定キー一覧[3], カスタム転送名.to_owned()),
        ]
    }
}
