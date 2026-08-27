//! 現在の作業ディレクトリの取得を1箇所へ閉じる。プロトコル通信の起動経路と`doctor`
//! サブコマンドの両方が同じ取得手続きを必要とするため、`std::env::current_dir`の
//! 直叩きをこの境界だけへ集約する（グローバルCLAUDE.md「暗黙のグローバル依存を
//! 関数の奥で直叩きしない」）。

use std::io;
use std::path::PathBuf;

pub(crate) fn 作業ディレクトリを解決する() -> io::Result<PathBuf> {
    std::env::current_dir()
}
