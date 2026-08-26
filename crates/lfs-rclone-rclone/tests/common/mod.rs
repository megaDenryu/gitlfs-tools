//! 偽rcloneを使う結合テストの共有ヘルパー。
//!
//! 偽rclone実行ファイル（`src/bin/fake_rclone.rs`）は環境変数を使わず、ファイルだけで
//! 挙動の指示を受け取る。ここではその指示置き場の作成・書き込み・読み出しと、
//! `Rclone保管庫`の組み立てをテストごとにまとめる。
//!
//! 注意: `tests/`配下の各テストファイルは個別の結合テストバイナリとしてコンパイルされ、
//! このモジュールを毎回別々にコンパイルする。どのバイナリも補助関数の全部は使わないため
//! `dead_code`警告が出る。これは共有テストヘルパーの既知の性質であり、本体クレートの
//! lint（`unwrap_used`等のdeny）を緩和するものではない
//! （`lfs-rclone-transfer/tests/common/mod.rs`と同じ扱い）。

#![allow(dead_code)]

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use lfs_rclone_domain::{一時ディレクトリ, 保管先基底パス, Rclone実行ファイルの場所, Rcloneリモート名, 転送タイムアウト};
use lfs_rclone_rclone::Rclone保管庫;

const 指示置き場の親: &str = "git_lfs_rclone_storage_fake_rclone_test";

/// 偽rclone実行ファイルの絶対パス。Cargoがこのクレートの`[[bin]] fake_rclone`用に
/// 設定する`CARGO_BIN_EXE_fake_rclone`を読む。
pub fn 偽rclone実行ファイルのパス() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_fake_rclone"))
}

/// テストごとに衝突しない「保管先基底パス」文字列を作る。偽rclone実行ファイルは
/// 引数中の保管先オブジェクトパスからこの文字列を復元し、指示置き場を特定する。
pub fn 固有の基底パス文字列を作る(接頭辞: &str) -> Result<String, Box<dyn std::error::Error>> {
    let 使い捨てディレクトリ = tempfile::tempdir()?;
    let 接尾辞 = 使い捨てディレクトリ
        .path()
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(|文字列| 文字列.into_owned())
        .unwrap_or_default();
    Ok(format!("{接頭辞}-{接尾辞}"))
}

/// 指示置き場。偽rclone実行ファイルの挙動をファイルで指示し、記録された引数を読み出す。
pub struct 偽rclone指示置き場 {
    ディレクトリ: PathBuf,
}

impl 偽rclone指示置き場 {
    pub fn 準備する(基底パス文字列: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let ディレクトリ = std::env::temp_dir().join(指示置き場の親).join(基底パス文字列);
        fs::create_dir_all(&ディレクトリ)?;
        Ok(Self { ディレクトリ })
    }

    /// 呼び出しを即座に終了させる。診断メッセージへの秘密混入を検査するテストでも使う。
    pub fn 即終了で応答させる(&self, 終了コード: i32, 標準出力: &str, 標準エラー: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(self.ディレクトリ.join("exit_code"), 終了コード.to_string())?;
        fs::write(self.ディレクトリ.join("stdout"), 標準出力)?;
        fs::write(self.ディレクトリ.join("stderr"), 標準エラー)?;
        Ok(())
    }

    pub fn 応答前に眠らせる(&self, 時間: Duration) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(self.ディレクトリ.join("sleep_ms"), 時間.as_millis().to_string())?;
        Ok(())
    }

    /// 最終オブジェクトが既に存在する状態を仕込む。`lsjson`はこのバイト数を返すようになる。
    pub fn 既存として仕込む(&self, バイト数: u64) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(self.ディレクトリ.join("present_size"), バイト数.to_string())?;
        fs::write(self.ディレクトリ.join("marker"), b"")?;
        Ok(())
    }

    /// `moveto`(最終化転送)が成功した後に`lsjson`が返すべきバイト数を仕込む。偽実行ファイルは
    /// 実際のバイト列を転送しないため、これがなければ最終化後のサイズを知りようがない。
    pub fn 最終化後のバイト数を仕込む(&self, バイト数: u64) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(self.ディレクトリ.join("finalize_size"), バイト数.to_string())?;
        Ok(())
    }

    pub fn マーカーが存在するか(&self) -> bool {
        self.ディレクトリ.join("marker").exists()
    }

    /// 記録済みの呼び出し引数を、呼び出し順に、引数配列として読み出す。
    pub fn 記録済み呼び出し一覧を読む(&self) -> Vec<Vec<String>> {
        let 内容 = fs::read_to_string(self.ディレクトリ.join("args_log")).unwrap_or_default();
        内容
            .lines()
            .map(|行| 行.split('\u{1f}').map(str::to_owned).collect())
            .collect()
    }
}

/// 偽rclone実行ファイルを指すよう組み立てた`Rclone保管庫`。
pub fn 偽rclone保管庫を作る(基底パス文字列: &str, タイムアウト: Duration) -> Result<Rclone保管庫, Box<dyn std::error::Error>> {
    let 実行ファイル = Rclone実行ファイルの場所::指定パスから生成する(偽rclone実行ファイルのパス());
    let リモート名 = Rcloneリモート名::生成する("fakeremote")?;
    let 基底パス = 保管先基底パス::生成する(基底パス文字列)?;
    let 一時ディレクトリ = 一時ディレクトリ::生成する(std::env::temp_dir());
    let タイムアウト = 転送タイムアウト::生成する(タイムアウト);
    Ok(Rclone保管庫::生成する(実行ファイル, リモート名, 基底パス, 一時ディレクトリ, タイムアウト))
}
