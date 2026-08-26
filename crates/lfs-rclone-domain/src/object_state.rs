//! 保管先における1オブジェクトの存在状態を表すドメインモデル。

use crate::expected_byte_count::期待バイト数;

/// 保管先における1オブジェクトの存在状態。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum オブジェクト状態 {
    未存在,
    存在,
    サイズの不一致 { 実サイズ: 期待バイト数 },
}
