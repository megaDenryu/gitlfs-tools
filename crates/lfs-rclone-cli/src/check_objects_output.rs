//! `check-objects`の表示行の組み立て。表示行を作るところまでを純粋な変換としてこの型が持ち、
//! 標準出力への書き出しは呼び出し側（`check_objects_command`）が行う
//! （`diagnostic_finding.rs`と同じ分け方）。

use lfs_rclone_transfer::{欠落オブジェクト, 点検報告};

use crate::git_lfs_file_listing::GitLFS追跡ファイル一覧;
use crate::object_check_scope::点検範囲;

const 片方向の突き合わせの注記: &str =
    "注記: この点検はGit LFSが参照するオブジェクトだけを見る。保管先には他のリポジトリが置いたオブジェクトも在るが、それは正常であり点検の対象ではない。";
const 欠落があるときの対処: &str =
    "対処: git lfs push --all origin <ブランチ名> を実行して実体を送り直す。再送は冪等であり、既に保管先に在るオブジェクトは転送されない。";

/// 1回の点検の結果を、利用者が読む行の並びへ変換するための値。
pub(crate) struct 点検結果の表示 {
    範囲: 点検範囲,
    追跡ファイル一覧: GitLFS追跡ファイル一覧,
    報告: 点検報告,
}

impl 点検結果の表示 {
    pub(crate) fn 生成する(範囲: 点検範囲, 追跡ファイル一覧: GitLFS追跡ファイル一覧, 報告: 点検報告) -> Self {
        Self { 範囲, 追跡ファイル一覧, 報告 }
    }

    pub(crate) fn 全て保管先に在るか(&self) -> bool {
        self.報告.全て保管先に在るか()
    }

    pub(crate) fn 表示行一覧(&self) -> Vec<String> {
        let mut 行一覧 = vec![
            format!("点検の範囲: {}", self.範囲.説明()),
            format!("点検したオブジェクト: {}件", self.報告.点検した件数()),
            format!("保管先に見つからないオブジェクト: {}件", self.報告.欠落一覧().len()),
        ];

        for 欠落 in self.報告.欠落一覧() {
            行一覧.push(self.欠落1件の行(欠落));
        }

        if self.報告.全て保管先に在るか() {
            行一覧.push("点検したオブジェクトはすべて保管先にある。".to_owned());
        } else {
            行一覧.push(欠落があるときの対処.to_owned());
        }
        行一覧.push(片方向の突き合わせの注記.to_owned());
        行一覧
    }

    fn 欠落1件の行(&self, 欠落: &欠落オブジェクト) -> String {
        let パス一覧 = self.追跡ファイル一覧.識別子を参照するパス一覧(欠落.識別子());
        let 参照元 = if パス一覧.is_empty() { "(参照元のファイルが不明)".to_owned() } else { パス一覧.join(", ") };
        format!("[欠落] {参照元} (oid: {}): {}", 欠落.識別子(), 欠落.事由())
    }
}
