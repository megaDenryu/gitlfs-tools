//! この実行ファイルの版を表す値型。版の出所をCargoのパッケージ版1つに固定し、
//! `version`サブコマンドの出力行と`doctor`の先頭行が同じ値を出すことを保証する。
//!
//! 版を上げるのは人であり、`Cargo.toml`の`[workspace.package] version`を書き換える。
//! この型は書き換えられた値を読むだけで、版を自分で決めない
//! （コード分割規約.md「値型」役割: 入出力を書いてはならない）。

/// ビルドしたときのCargoのパッケージ版。
pub(crate) struct この実行ファイルの版(&'static str);

impl この実行ファイルの版 {
    pub(crate) fn cargoのパッケージ版から生成する() -> Self {
        Self(env!("CARGO_PKG_VERSION"))
    }

    /// 版だけの綴り（`1.0.0`）。`doctor`が項目名の後ろへ置く。
    pub(crate) fn 版の文字列(&self) -> &'static str {
        self.0
    }

    /// プログラム名を添えた1行（`gitlfs-tools 1.0.0`）。`version`サブコマンドが標準出力へ書く。
    pub(crate) fn プログラム名と版の1行を組み立てる(&self) -> String {
        format!("gitlfs-tools {}", self.0)
    }
}
