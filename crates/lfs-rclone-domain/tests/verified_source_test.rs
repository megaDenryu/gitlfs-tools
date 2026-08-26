//! `検証済み転送元`の生成条件（バイト数とSHA-256の両方一致）のテスト。

use std::io::Write;

use lfs_rclone_domain::{オブジェクト識別子, 保管エラー, 整合性エラー, 検証前のローカルファイル, 期待バイト数};
use sha2::{Digest, Sha256};

fn 内容を書いた一時ファイルを作る(内容: &[u8]) -> Result<tempfile::NamedTempFile, Box<dyn std::error::Error>> {
    let mut ファイル = tempfile::NamedTempFile::new()?;
    ファイル.write_all(内容)?;
    Ok(ファイル)
}

fn 内容から識別子を計算する(内容: &[u8]) -> Result<オブジェクト識別子, Box<dyn std::error::Error>> {
    let ダイジェスト = Sha256::digest(内容);
    let 十六進文字列: String = ダイジェスト.iter().map(|バイト| format!("{バイト:02x}")).collect();
    Ok(オブジェクト識別子::生成する(&十六進文字列)?)
}

#[test]
fn バイト数とハッシュが両方一致すれば検証済み転送元を生成できる() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"git-lfs-rclone-storage issue3 fixture";
    let ファイル = 内容を書いた一時ファイルを作る(内容)?;
    let 識別子 = 内容から識別子を計算する(内容)?;
    let バイト数 = 期待バイト数::生成する(u64::try_from(内容.len())?);

    let 検証前 = 検証前のローカルファイル::生成する(ファイル.path());
    let 検証結果 = 検証前.検証する(&識別子, バイト数);

    assert!(検証結果.is_ok());
    Ok(())
}

#[test]
fn バイト数だけ一致してもハッシュが違えば失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"git-lfs-rclone-storage issue3 fixture";
    let ファイル = 内容を書いた一時ファイルを作る(内容)?;
    let 実際のバイト数 = 期待バイト数::生成する(u64::try_from(内容.len())?);
    let 別内容の識別子 = 内容から識別子を計算する(b"a completely different fixture content")?;

    let 検証前 = 検証前のローカルファイル::生成する(ファイル.path());
    let 検証結果 = 検証前.検証する(&別内容の識別子, 実際のバイト数);

    assert!(matches!(
        検証結果,
        Err(保管エラー::整合性(整合性エラー::内容ハッシュが不一致 { .. }))
    ));
    Ok(())
}

#[test]
fn ハッシュだけ一致してもバイト数が違えば失敗する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = b"git-lfs-rclone-storage issue3 fixture";
    let ファイル = 内容を書いた一時ファイルを作る(内容)?;
    let 識別子 = 内容から識別子を計算する(内容)?;
    let 違うバイト数 = 期待バイト数::生成する(u64::try_from(内容.len())? + 1);

    let 検証前 = 検証前のローカルファイル::生成する(ファイル.path());
    let 検証結果 = 検証前.検証する(&識別子, 違うバイト数);

    assert!(matches!(
        検証結果,
        Err(保管エラー::整合性(整合性エラー::バイト数が不一致 { .. }))
    ));
    Ok(())
}
