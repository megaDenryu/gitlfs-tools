//! agentを子プロセスとして起動し、stdin/stdoutでprotocol JSONを往復させるテスト専用の
//! 薄いラッパー。標準エラー出力は別スレッドで読み切り、標準出力の読み取りをブロックさせない
//! （`gitlfs-tools-rclone`の`Rcloneプロセス実行器`と同じ理由）。

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::thread::{self, JoinHandle};

pub struct プロトコルテストプロセス {
    子プロセス: Child,
    標準入力: ChildStdin,
    標準出力読み取り: BufReader<std::process::ChildStdout>,
    標準エラー出力読み取り: JoinHandle<String>,
}

impl プロトコルテストプロセス {
    pub fn 起動する(
        実行ファイル: &Path,
        作業ディレクトリ: &Path,
        pc設定ディレクトリ: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut 子プロセス = Command::new(実行ファイル)
            .current_dir(作業ディレクトリ)
            .env("GITLFS_TOOLS_PC_CONFIG_DIR", pc設定ディレクトリ)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let 標準入力 = 子プロセス.stdin.take().ok_or("標準入力の取得に失敗しました")?;
        let 標準出力 = 子プロセス.stdout.take().ok_or("標準出力の取得に失敗しました")?;
        let 標準エラー出力 = 子プロセス.stderr.take().ok_or("標準エラー出力の取得に失敗しました")?;
        let 標準エラー出力読み取り = thread::spawn(move || {
            let mut 標準エラー出力 = 標準エラー出力;
            let mut バッファ = String::new();
            let _ = std::io::Read::read_to_string(&mut 標準エラー出力, &mut バッファ);
            バッファ
        });

        Ok(Self { 子プロセス, 標準入力, 標準出力読み取り: BufReader::new(標準出力), 標準エラー出力読み取り })
    }

    pub fn 一行送る(&mut self, 値: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.標準入力, "{値}")?;
        self.標準入力.flush()?;
        Ok(())
    }

    /// stdoutから1行読み、単一行の有効なJSONとして解析する。
    pub fn 一行受け取る(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut 行 = String::new();
        let 読み取りバイト数 = self.標準出力読み取り.read_line(&mut 行)?;
        if 読み取りバイト数 == 0 {
            return Err("標準出力がEOFになりました".into());
        }
        Ok(serde_json::from_str(行.trim_end())?)
    }

    /// 標準入力を閉じ、子プロセスの終了を待ち、標準エラー出力の全文を回収する。
    pub fn 終了を待って後始末する(self) -> Result<(ExitStatus, String), Box<dyn std::error::Error>> {
        drop(self.標準入力);
        let mut 子プロセス = self.子プロセス;
        let 終了状態 = 子プロセス.wait()?;
        let 標準エラー出力 = self.標準エラー出力読み取り.join().map_err(|_| "標準エラー出力の読み取りスレッドが失敗しました")?;
        Ok((終了状態, 標準エラー出力))
    }
}
