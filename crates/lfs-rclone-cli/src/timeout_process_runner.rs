//! `保管先書き込み確認`が使う、転送タイムアウトを尊重した子プロセス実行の外部境界。
//! `lfs-rclone-rclone`crateの`Rcloneプロセス実行器`と同じ責務を持つが、そちらは
//! `pub(crate)`でクレートをまたいで参照できないため、doctorの書き込み確認専用として
//! ここへ複製する（このタスクで触ってよい範囲が`lfs-rclone-cli`配下に限られるため）。
//! 出力の内容はどの呼び出し元も使わないため`Stdio::null()`で捨て、標準出力を捕捉して
//! 読み切るスレッドは持たない（読み取り待ちに起因するデッドロックの心配がそもそも無い）。

use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use lfs_rclone_domain::{保管エラー, Rclone実行ファイルの場所, 転送タイムアウト};

const 監視間隔: Duration = Duration::from_millis(20);

/// rclone実行ファイルの指定とタイムアウトを保持し、rcloneの起動を行うサービス。
pub(crate) struct タイムアウト付きrclone実行器 {
    実行ファイル: Rclone実行ファイルの場所,
    タイムアウト: 転送タイムアウト,
}

impl タイムアウト付きrclone実行器 {
    pub(crate) fn 生成する(実行ファイル: Rclone実行ファイルの場所, タイムアウト: 転送タイムアウト) -> Self {
        Self { 実行ファイル, タイムアウト }
    }

    /// 指定した引数でrcloneを起動し、正常終了するまで待つ。タイムアウトを過ぎたら
    /// 強制終了して失敗として扱う。
    pub(crate) fn 実行する(&self, 引数: &[&str]) -> Result<(), 保管エラー> {
        let mut 子プロセス = Command::new(self.実行ファイル.プログラム名())
            .args(引数)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|エラー| 保管エラー::子プロセス { 説明: format!("rcloneの起動に失敗しました: {エラー}") })?;

        match self.終了を待つ(&mut 子プロセス) {
            Ok(状態) if 状態.success() => Ok(()),
            Ok(状態) => Err(保管エラー::子プロセス { 説明: format!("終了コード{:?}で失敗しました", 状態.code()) }),
            Err(エラー) => {
                let _ = 子プロセス.kill();
                let _ = 子プロセス.wait();
                Err(エラー)
            }
        }
    }

    fn 終了を待つ(&self, 子プロセス: &mut Child) -> Result<ExitStatus, 保管エラー> {
        let 開始時刻 = Instant::now();
        loop {
            if let Some(状態) = 子プロセス
                .try_wait()
                .map_err(|エラー| 保管エラー::子プロセス { 説明: format!("終了待ちに失敗しました: {エラー}") })?
            {
                return Ok(状態);
            }
            if 開始時刻.elapsed() >= self.タイムアウト.値() {
                return Err(保管エラー::子プロセス { 説明: "タイムアウトしました".to_owned() });
            }
            thread::sleep(監視間隔);
        }
    }
}
