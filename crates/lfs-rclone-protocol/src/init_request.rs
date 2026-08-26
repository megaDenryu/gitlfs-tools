//! `init`要求を表す値型。

use crate::transfer_operation_kind::転送操作種別;

/// `init`要求。`operation`から解決した転送方向だけを持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 初期化要求 {
    操作種別: 転送操作種別,
}

impl 初期化要求 {
    pub(crate) fn 生成する(操作種別: 転送操作種別) -> Self {
        Self { 操作種別 }
    }

    pub(crate) fn 操作種別(&self) -> 転送操作種別 {
        self.操作種別
    }
}
