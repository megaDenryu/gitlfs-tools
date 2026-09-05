//! Git LFS custom transfer protocolの要求ループ本体。standalone agentとして
//! stdin/stdoutを処理する（Issue #7）。
//!
//! `upload`・`download`1件ごとの処理内容は`protocol_session_transfer.rs`が持つ
//! （同じ`プロトコルセッション`型への別`impl`。ループ制御と1件ごとの転送処理は
//! 別々に名前を付けられる責務であり、行数の分割ではない）。

use std::io::{self, BufRead};

use crate::exit_status::終了状態;
use crate::presentable_error::表示用エラー;
use crate::protocol_request::{行から要求を解析する, 転送プロトコル要求};
use crate::protocol_response_writer::プロトコル応答送信器;
use crate::transfer_session_boundary::転送セッション開始境界;

/// プロトコルの要求ループを保持するサービス。`境界`の実装（`gitlfs-tools-cli`が持つ）を
/// コンストラクタで受け取り保持する（グローバルCLAUDE.md「サービスの依存保持と
/// コールバックの限定」）。
pub struct プロトコルセッション<境界: 転送セッション開始境界> {
    pub(crate) 境界: 境界,
    pub(crate) 応答: プロトコル応答送信器,
}

impl<境界: 転送セッション開始境界> プロトコルセッション<境界> {
    pub fn 生成する(境界: 境界) -> Self {
        Self { 境界, 応答: プロトコル応答送信器::生成する() }
    }

    /// stdinを読み切るまで要求を処理する。
    pub fn 実行する(&self) -> 終了状態 {
        let 標準入力 = io::stdin();
        let mut 行一覧 = 標準入力.lock().lines();

        match self.初期化する(&mut 行一覧) {
            Ok(Some(セッション)) => self.要求ループを回す(&mut 行一覧, &セッション),
            Ok(None) => 終了状態::正常終了,
            Err(終了状態) => 終了状態,
        }
    }

    /// 最初の要求を読み、`init`として処理する。stdinが即座にEOFなら`Ok(None)`。
    fn 初期化する(
        &self,
        行一覧: &mut impl Iterator<Item = io::Result<String>>,
    ) -> Result<Option<境界::開始済み転送セッション>, 終了状態> {
        let Some(行) = 行一覧.next() else { return Ok(None) };
        let 行 = 行.map_err(|エラー| {
            eprintln!("標準入力の読み取りに失敗しました: {エラー}");
            終了状態::継続不能な失敗
        })?;

        let 転送プロトコル要求::初期化(初期化要求) = 行から要求を解析する(&行).map_err(|エラー| {
            self.送信結果をログする(self.応答.初期化失敗を送る(&表示用エラー::from(&エラー)));
            eprintln!("initの解析に失敗しました: {エラー}");
            終了状態::継続不能な失敗
        })?
        else {
            eprintln!("最初の要求がinitではありませんでした");
            return Err(終了状態::継続不能な失敗);
        };

        match self.境界.開始する(初期化要求.操作種別()) {
            Ok(セッション) => {
                self.送信結果をログする(self.応答.初期化成功を送る());
                Ok(Some(セッション))
            }
            Err(エラー) => {
                self.送信結果をログする(self.応答.初期化失敗を送る(&表示用エラー::from(&エラー)));
                eprintln!("initに失敗しました: {エラー}");
                Err(終了状態::継続不能な失敗)
            }
        }
    }

    /// `init`後の要求を`terminate`または入力終了まで処理し続ける。
    ///
    /// 注意: Git LFS custom transfer protocolは1要求1応答の同期プロトコルであり、
    /// Git LFS側は書いた行の応答を待ってから次を送る。この関数がループを回すどの分岐でも
    /// 応答行を1つも書かずに次の読み取りへ進んではならない（無応答のまま進むと、応答を
    /// 待つGit LFS側がハングする）。
    fn 要求ループを回す(
        &self,
        行一覧: &mut impl Iterator<Item = io::Result<String>>,
        セッション: &境界::開始済み転送セッション,
    ) -> 終了状態 {
        for 行 in 行一覧 {
            let Ok(行) = 行 else {
                eprintln!("標準入力の読み取りに失敗しました");
                return 終了状態::継続不能な失敗;
            };

            match 行から要求を解析する(&行) {
                Ok(転送プロトコル要求::終了) => return 終了状態::正常終了,
                Ok(転送プロトコル要求::初期化(_)) => {
                    eprintln!("initはセッション開始後に再送されました。既存セッションを維持したまま成功応答を返します。");
                    self.送信結果をログする(self.応答.初期化成功を送る());
                }
                Ok(転送プロトコル要求::アップロード { oid, size, path }) => {
                    self.アップロードを処理する(セッション, &oid, size, &path);
                }
                Ok(転送プロトコル要求::ダウンロード { oid, size }) => {
                    self.ダウンロードを処理する(セッション, &oid, size);
                }
                Err(エラー) => {
                    eprintln!("要求の解析に失敗しました: {エラー}");
                    return 終了状態::継続不能な失敗;
                }
            }
        }
        // ここへ落ちるのはstdinがEOFになった場合である。
        // 注意: terminateを受け取る前にGit LFS側が標準入力を閉じた場合も正常終了として扱う。
        // Git LFS側が既にプロセスを終えていれば非0で終わっても応答を受け取れる相手がおらず
        // 得るものがない。この時点で未処理のまま所有している一時資源も無い（各upload/download
        // 要求は1行読むごとに完結し、失敗時の一時ファイルもその場で後始末される）。
        終了状態::正常終了
    }

    /// 標準出力への書き込み失敗を握りつぶさず、少なくとも標準エラー出力へ残す。
    pub(crate) fn 送信結果をログする(&self, 結果: io::Result<()>) {
        if let Err(エラー) = 結果 {
            eprintln!("標準出力への書き込みに失敗しました: {エラー}");
        }
    }
}
