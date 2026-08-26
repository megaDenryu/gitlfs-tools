//! rcloneを子プロセスとして起動し、標準出力・標準エラー出力を捕捉し、タイムアウトを
//! 監視する外部境界。lsjson・copyto・movetoの意味は知らない（アーキテクチャ.md 判断6
//! 「標準出力はGit LFSとの通信専用にする」を守るため、子プロセスの出力はこの型の外へ
//! 漏らさず戻り値として返すだけにする）。
//!
//! 注意: `std::process::Command`はタイムアウトを持たない。外部crateを足さず、
//! `try_wait`を短い間隔でポーリングして期限超過を検出し、超過したら`kill`する方式で
//! 実現する。子プロセスの標準出力・標準エラー出力は別スレッドで並行して読み切る。
//! そうしないと、出力量がOSパイプの容量を超えたときに子プロセスが書き込みで止まり、
//! 親が`wait`系の呼び出しで待ち続ける典型的なデッドロックを踏む。

use std::ffi::OsString;
use std::io::Read;
use std::process::{Child, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::rclone_executable::Rclone実行ファイル;
use crate::rclone_execution_error::Rclone実行エラー;
use crate::rclone_operation::Rclone操作;
use crate::transfer_timeout::転送タイムアウト;

const 監視間隔: Duration = Duration::from_millis(20);

/// 実行ファイルの指定とタイムアウトを保持し、rcloneの起動を行うサービス。
pub(crate) struct Rcloneプロセス実行器 {
    実行ファイル: Rclone実行ファイル,
    タイムアウト: 転送タイムアウト,
}

impl Rcloneプロセス実行器 {
    pub(crate) fn 生成する(実行ファイル: Rclone実行ファイル, タイムアウト: 転送タイムアウト) -> Self {
        Self { 実行ファイル, タイムアウト }
    }

    /// 指定した引数でrcloneを起動し、標準出力を待ち切って返す。標準エラー出力は捕捉して
    /// 読み切るが、秘密情報の混入を避けるため戻り値にも診断メッセージにも含めない。
    pub(crate) fn 実行する(&self, 操作: Rclone操作, 引数: &[OsString]) -> Result<Vec<u8>, Rclone実行エラー> {
        let mut 子プロセス = self
            .実行ファイル
            .コマンドを生成する()
            .args(引数)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|エラー| Rclone実行エラー::起動失敗 { 操作, 説明: エラー.to_string() })?;

        let 標準出力ハンドル = 子プロセス
            .stdout
            .take()
            .ok_or_else(|| Rclone実行エラー::起動失敗 { 操作, 説明: "標準出力の取得に失敗しました".to_owned() })?;
        let 標準エラーハンドル = 子プロセス
            .stderr
            .take()
            .ok_or_else(|| Rclone実行エラー::起動失敗 { 操作, 説明: "標準エラー出力の取得に失敗しました".to_owned() })?;

        let 標準出力読み取り = thread::spawn(move || 読み取り切る(標準出力ハンドル));
        let 標準エラー読み取り = thread::spawn(move || 読み取り切る(標準エラーハンドル));

        let 終了状態 = match self.終了を待つ(&mut 子プロセス, 操作) {
            Ok(状態) => 状態,
            Err(タイムアウト) => {
                let _ = 子プロセス.kill();
                let _ = 子プロセス.wait();
                let _ = 標準出力読み取り.join();
                let _ = 標準エラー読み取り.join();
                return Err(タイムアウト);
            }
        };

        let 標準出力 = 標準出力読み取り.join().unwrap_or_default();
        let _標準エラー出力 = 標準エラー読み取り.join().unwrap_or_default();

        match 終了状態.code() {
            Some(0) => Ok(標準出力),
            コード => Err(Rclone実行エラー::非0終了 { 操作, 終了コード: コード }),
        }
    }

    /// 期限まで`try_wait`をポーリングする。期限を過ぎたら`タイムアウト`エラーを返す。
    /// 呼び出し側が子プロセスの後片づけ（kill・読み取りスレッドの合流）を行う。
    fn 終了を待つ(&self, 子プロセス: &mut Child, 操作: Rclone操作) -> Result<std::process::ExitStatus, Rclone実行エラー> {
        let 開始時刻 = Instant::now();
        loop {
            if let Some(状態) = 子プロセス
                .try_wait()
                .map_err(|エラー| Rclone実行エラー::起動失敗 { 操作, 説明: エラー.to_string() })?
            {
                return Ok(状態);
            }
            if 開始時刻.elapsed() >= self.タイムアウト.値() {
                return Err(Rclone実行エラー::タイムアウト { 操作 });
            }
            thread::sleep(監視間隔);
        }
    }
}

fn 読み取り切る(mut ハンドル: impl Read) -> Vec<u8> {
    let mut バッファ = Vec::new();
    let _ = ハンドル.read_to_end(&mut バッファ);
    バッファ
}
