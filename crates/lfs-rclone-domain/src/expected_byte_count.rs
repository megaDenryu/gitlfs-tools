//! 転送対象オブジェクトのバイト数を表す値型。

/// バイト数のnewtype。期待値・実測値のどちらの役割にも使う。
/// 役割の違いはこの型自体でなく、束縛する変数名・フィールド名で表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct 期待バイト数(u64);

impl 期待バイト数 {
    pub fn 生成する(値: u64) -> Self {
        Self(値)
    }

    /// rcloneの引数組み立て等、外部境界へ渡すための数値。
    pub fn 値(&self) -> u64 {
        self.0
    }
}
