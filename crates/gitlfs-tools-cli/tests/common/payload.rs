//! アップロード対象ファイルの作成とOID計算。設定ファイル一式（`fixtures.rs`）とは別の
//! 責務（転送対象データの用意）のため別ファイルに分ける。

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// 指定ディレクトリ内へ内容を書いた一時ファイルを作り、そのパスとOIDとバイト数を返す。
pub fn アップロード元を作る(
    格納先ディレクトリ: &Path,
    ファイル名: &str,
    内容: &[u8],
) -> Result<(PathBuf, String, u64), Box<dyn std::error::Error>> {
    let パス = 格納先ディレクトリ.join(ファイル名);
    std::fs::write(&パス, 内容)?;
    let ダイジェスト = Sha256::digest(内容);
    let oid: String = ダイジェスト.iter().map(|バイト| format!("{バイト:02x}")).collect();
    Ok((パス, oid, u64::try_from(内容.len())?))
}
