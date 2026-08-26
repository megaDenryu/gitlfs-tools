//! 標準出力へGit LFS protocolのJSONを1行書き、直後にflushする外部境界。
//!
//! 注意: このcrateで`println!`または標準出力への直接書き込みを行ってよいのはこの型だけ
//! である（CLAUDE.md「標準出力の規律」）。rcloneの標準出力はこの型を経由せず、
//! `lfs-rclone-rclone`が別途捕捉して外へ漏らさない。

use std::io::{self, Write};

use serde::Serialize;

/// 標準出力へprotocol JSONを書く外部境界。状態を持たず、呼び出しのたびに標準出力を
/// ロックして1行書き切る。
pub(crate) struct 標準出力書き込み器;

impl 標準出力書き込み器 {
    pub(crate) fn 生成する() -> Self {
        Self
    }

    /// 値を1行のJSONとして書き、直後にflushする。
    pub(crate) fn 一行書く(&self, 値: &impl Serialize) -> io::Result<()> {
        let 本文 = serde_json::to_string(値).map_err(io::Error::other)?;
        let mut 標準出力 = io::stdout().lock();
        writeln!(標準出力, "{本文}")?;
        標準出力.flush()
    }
}
