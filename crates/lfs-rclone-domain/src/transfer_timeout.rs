//! rcloneの子プロセスを起動してから応答を待つ上限時間を表す値型。

use std::time::Duration;

/// 転送タイムアウト。この時間を超えても子プロセスが終了しなければ強制終了する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct 転送タイムアウト(Duration);

impl 転送タイムアウト {
    pub fn 生成する(上限時間: Duration) -> Self {
        Self(上限時間)
    }

    pub fn 値(&self) -> Duration {
        self.0
    }
}
