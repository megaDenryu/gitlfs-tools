//! `プロトコルセッション`のうち、`upload`・`download`1件ごとの処理だけを担う`impl`。
//! ループ制御は`protocol_session.rs`が持つ。1件の失敗はここで完結させ、呼び出し元の
//! ループを止めない（Issue #2 3.3節・3.4節「1 objectの失敗だけでprocessを終了しない」）。

use gitlfs_tools_domain::検証前のローカルファイル;
use gitlfs_tools_transfer::{アップロード要求, ダウンロード要求};

use crate::presentable_error::表示用エラー;
use crate::protocol_session::プロトコルセッション;
use crate::transfer_session_boundary::{開始済み転送セッション, 転送セッション開始境界};

impl<境界: 転送セッション開始境界> プロトコルセッション<境界> {
    pub(crate) fn アップロードを処理する(&self, セッション: &境界::開始済み転送セッション, oid: &str, size: u64, path: &str) {
        let 入力ファイル = 検証前のローカルファイル::生成する(path);
        let 要求 = match アップロード要求::生成する(oid, size, 入力ファイル) {
            Ok(要求) => 要求,
            Err(エラー) => {
                self.送信結果をログする(self.応答.失敗完了を送る(oid, &表示用エラー::from(&エラー)));
                return;
            }
        };

        match セッション.アップロードする(要求) {
            Ok(完了) => {
                self.送信結果をログする(self.応答.進捗を送る(oid, size));
                self.送信結果をログする(self.応答.アップロード完了を送る(完了.識別子().文字列表現()));
            }
            Err(エラー) => self.送信結果をログする(self.応答.失敗完了を送る(oid, &表示用エラー::from(&エラー))),
        }
    }

    pub(crate) fn ダウンロードを処理する(&self, セッション: &境界::開始済み転送セッション, oid: &str, size: u64) {
        let 要求 = match ダウンロード要求::生成する(oid, size) {
            Ok(要求) => 要求,
            Err(エラー) => {
                self.送信結果をログする(self.応答.失敗完了を送る(oid, &表示用エラー::from(&エラー)));
                return;
            }
        };

        match セッション.ダウンロードする(要求) {
            Ok(完了) => {
                self.送信結果をログする(self.応答.進捗を送る(oid, size));
                let パス = 完了.パス().to_string_lossy().into_owned();
                self.送信結果をログする(self.応答.ダウンロード完了を送る(完了.識別子().文字列表現(), &パス));
            }
            Err(エラー) => self.送信結果をログする(self.応答.失敗完了を送る(oid, &表示用エラー::from(&エラー))),
        }
    }
}
