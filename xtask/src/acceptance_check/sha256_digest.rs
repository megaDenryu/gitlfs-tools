//! ファイルのSHA-256とバイト数を独立に実測する外部境界。xtaskは外部依存ゼロ（std
//! のみ）のため、`sha2`crateを足さずWindows標準の`certutil`を子プロセスとして使う。
//! 転送経路（agent・rclone）が計算するダイジェストと同じ値を、別経路で確かめるための
//! 検証専用手段である。

use std::path::Path;
use std::process::Command;

/// 1ファイルの内容を指紋づけた値。パスは持たず、内容だけを表す。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ファイル指紋 {
    pub バイト数: u64,
    pub sha256十六進: String,
}

impl ファイル指紋 {
    pub fn 計測する(対象: &Path) -> Result<Self, String> {
        let バイト数 = std::fs::metadata(対象).map_err(|失敗| format!("{}のメタデータを取得できなかった: {失敗}", 対象.display()))?.len();
        let sha256十六進 = certutilでsha256を計測する(対象)?;
        Ok(Self { バイト数, sha256十六進 })
    }
}

fn certutilでsha256を計測する(対象: &Path) -> Result<String, String> {
    let 出力 = Command::new("certutil")
        .args(["-hashfile"])
        .arg(対象)
        .arg("SHA256")
        .output()
        .map_err(|失敗| format!("certutilを起動できなかった: {失敗}"))?;
    if !出力.status.success() {
        return Err(format!("certutilが失敗した({})", 対象.display()));
    }
    let 標準出力 = String::from_utf8_lossy(&出力.stdout);
    標準出力
        .lines()
        .map(str::trim)
        .find(|行| 行.len() == 64 && 行.bytes().all(|バイト| バイト.is_ascii_hexdigit()))
        .map(str::to_lowercase)
        .ok_or_else(|| format!("certutilの出力から64桁の16進文字列を取り出せなかった({})", 対象.display()))
}
