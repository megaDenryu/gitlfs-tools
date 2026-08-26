//! `プロトコルセッション::実行する`の戻り値を表す値型。
//!
//! 生のプロセス終了コード（`i32`）をこのcrateの外まで持ち出さない。`std::process::ExitCode`
//! への変換はコンポジションルート（`lfs-rclone-cli`のmain）という境界1箇所だけで行う。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 終了状態 {
    正常終了,
    継続不能な失敗,
}

impl 終了状態 {
    pub fn 正常か(&self) -> bool {
        matches!(self, Self::正常終了)
    }
}
