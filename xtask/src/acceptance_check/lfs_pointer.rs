//! Git LFS pointerファイルの本文を扱う純粋関数群。副作用も依存も持たないため自由関数として
//! 置く（グローバルCLAUDE.md「サービスの依存保持とコールバックの限定」条1）。

pub fn pointer形式か(本文: &str) -> bool {
    本文.starts_with("version https://git-lfs.github.com/spec/v1")
}

/// pointer本文から`oid sha256:<64桁16進>`と`size <バイト数>`を取り出す。
pub fn oidとサイズを取り出す(pointer本文: &str) -> Result<(String, u64), String> {
    if !pointer形式か(pointer本文) {
        return Err(format!("pointer形式ではない本文だった: {pointer本文:?}"));
    }
    let oid = pointer本文
        .lines()
        .find_map(|行| 行.strip_prefix("oid sha256:"))
        .ok_or("pointer本文にoid行がない")?
        .trim()
        .to_owned();
    if oid.len() != 64 || !oid.bytes().all(|バイト| バイト.is_ascii_hexdigit()) {
        return Err(format!("oidが64桁の16進文字列ではない: {oid}"));
    }
    let size = pointer本文
        .lines()
        .find_map(|行| 行.strip_prefix("size "))
        .ok_or("pointer本文にsize行がない")?
        .trim()
        .parse::<u64>()
        .map_err(|失敗| format!("size行を数値化できなかった: {失敗}"))?;
    Ok((oid, size))
}
