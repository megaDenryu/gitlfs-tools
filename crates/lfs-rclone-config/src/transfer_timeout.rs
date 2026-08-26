//! rclone転送のタイムアウトを表す値型。

use std::time::Duration;

/// PC設定`transfer_timeout_seconds`から解決した転送タイムアウト。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct 転送タイムアウト(Duration);

impl 転送タイムアウト {
    pub fn 秒数から生成する(秒数: u64) -> Self {
        Self(Duration::from_secs(秒数))
    }

    /// rcloneアダプタの起動待ち時間等、外部境界へ渡すための値。
    pub fn 値(&self) -> Duration {
        self.0
    }
}
