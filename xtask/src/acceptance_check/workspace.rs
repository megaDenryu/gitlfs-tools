//! 受入試験1回分の一時作業域。`tempfile`crateはxtaskから使えないため、
//! `std::env::temp_dir()`の下に固有の名前で作り、終了時に明示的に片づける
//! （親からの指示「一時ディレクトリは…固有の名前で作り、終了時に片づける」）。

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct 一時作業域(PathBuf);

impl 一時作業域 {
    pub fn 作成する() -> Result<Self, String> {
        let 固有名 = format!("gitlfs-tools-accept-{}-{}", std::process::id(), 現在時刻のナノ秒());
        let ルート = std::env::temp_dir().join(固有名);
        std::fs::create_dir_all(&ルート).map_err(|失敗| format!("{}を作成できなかった: {失敗}", ルート.display()))?;
        Ok(Self(ルート))
    }

    pub fn ルート(&self) -> &Path {
        &self.0
    }

    /// 名前付きの子ディレクトリを作って返す（`temp_directory`のように事前存在が要る先）。
    pub fn 子ディレクトリ(&self, 名前: &str) -> Result<PathBuf, String> {
        let パス = self.0.join(名前);
        std::fs::create_dir_all(&パス).map_err(|失敗| format!("{}を作成できなかった: {失敗}", パス.display()))?;
        Ok(パス)
    }

    /// 未作成のまま子パスだけを組み立てる（`git init`等、呼び出し先が自分で作る場合）。
    pub fn 子パス(&self, 名前: &str) -> PathBuf {
        self.0.join(名前)
    }

    /// 作業域全体を削除する。片づけの失敗が本来の失敗を隠さないよう、独立した
    /// `Result`として返す（呼び出し元がすでに得た検査結果とは別に報告する）。
    pub fn 後始末する(self) -> Result<(), String> {
        if self.0.is_dir() {
            std::fs::remove_dir_all(&self.0).map_err(|失敗| format!("{}を削除できなかった: {失敗}", self.0.display()))
        } else {
            Ok(())
        }
    }
}

fn 現在時刻のナノ秒() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|経過| 経過.as_nanos()).unwrap_or(0)
}
