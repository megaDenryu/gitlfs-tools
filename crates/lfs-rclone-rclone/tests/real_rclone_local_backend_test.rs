//! 実rcloneプロセスとの結合テスト。資格情報が不要なrcloneのlocal backendへ、
//! アップロード→存在確認→ダウンロードを実際に往復させる。
//!
//! 実行ファイルの場所は環境変数`LFS_RCLONE_TEST_EXECUTABLE`で受け取る。未設定、または
//! 指すファイルが存在しなければ、テストを失敗させずに読み飛ばした旨を標準エラー出力へ
//! 書いて成功扱いで終える（実rcloneが入っていない環境でも`cargo xtask verify`が
//! 通るようにするため）。
//!
//! 注意: named remoteやrclone設定ファイルを一切使わない。一時ディレクトリの絶対パスを
//! `<ドライブ文字>:<残りのパス(/区切り)>`へ分解し、ドライブ文字をそのまま`Rcloneリモート名`
//! として渡す。rclone はこの1文字のリモート名をWindowsのドライブ文字と同じパターンとして
//! 認識し、素の絶対パス（local backend）として扱う。これにより名前付きremoteの設定なしで
//! 実際のファイルシステムへの往復を検証できる。

mod common;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use lfs_rclone_domain::{
    一時ディレクトリ, オブジェクト状態, オブジェクト識別子, 保管先基底パス, 期待バイト数, Rclone実行ファイルの場所, Rcloneリモート名,
    転送タイムアウト, 検証前のローカルファイル,
};
use lfs_rclone_rclone::Rclone保管庫;
use lfs_rclone_storage_port::{アップロード結果, オブジェクト保管庫};
use sha2::{Digest, Sha256};

fn rclone実行ファイルのパスを探す() -> Option<PathBuf> {
    let パス = std::env::var("LFS_RCLONE_TEST_EXECUTABLE").ok()?;
    let パス = PathBuf::from(パス);
    パス.is_file().then_some(パス)
}

fn ドライブとパスへ分ける(絶対パス: &Path) -> Result<(String, String), Box<dyn std::error::Error>> {
    let 文字列 = 絶対パス.to_str().ok_or("一時ディレクトリのパスがUTF-8ではありません")?;
    let 正規化 = 文字列.replace('\\', "/");
    let (ドライブ, 残り) = 正規化.split_once(':').ok_or("絶対パスにドライブ文字がありません")?;
    Ok((ドライブ.to_owned(), 残り.to_owned()))
}

#[test]
fn 実rcloneのlocal_backendでアップロードから存在確認ダウンロードまで往復する() -> Result<(), Box<dyn std::error::Error>> {
    let Some(実行ファイルパス) = rclone実行ファイルのパスを探す() else {
        eprintln!(
            "LFS_RCLONE_TEST_EXECUTABLE が未設定、または指すファイルが見つからないため、実rcloneとの結合テストを読み飛ばします。"
        );
        return Ok(());
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = ドライブとパスへ分ける(保管先ルート.path())?;

    let 実行ファイル = Rclone実行ファイルの場所::指定パスから生成する(実行ファイルパス.clone());
    let リモート名 = Rcloneリモート名::生成する(ドライブ)?;
    let 基底パス = 保管先基底パス::生成する(残り)?;
    let 作業ディレクトリ = tempfile::tempdir()?;
    let 一時ディレクトリ = 一時ディレクトリ::生成する(作業ディレクトリ.path());
    let タイムアウト = 転送タイムアウト::生成する(Duration::from_secs(30));
    let 保管庫 = Rclone保管庫::生成する(実行ファイル, リモート名, 基底パス, 一時ディレクトリ, タイムアウト);

    let 内容 = b"git-lfs-rclone-storage issue5 real rclone local backend roundtrip";
    let ダイジェスト = Sha256::digest(内容);
    let 十六進文字列: String = ダイジェスト.iter().map(|バイト| format!("{バイト:02x}")).collect();
    let 識別子 = オブジェクト識別子::生成する(&十六進文字列)?;
    let バイト数 = 期待バイト数::生成する(u64::try_from(内容.len())?);

    let mut アップロード元ファイル = tempfile::NamedTempFile::new()?;
    アップロード元ファイル.write_all(内容)?;
    let 検証前 = 検証前のローカルファイル::生成する(アップロード元ファイル.path());
    let 検証済み = 検証前.検証する(&識別子, バイト数)?;

    let 転送前状態 = 保管庫.存在を確認する(&識別子, バイト数)?;
    assert_eq!(転送前状態, オブジェクト状態::未存在);

    let 初回結果 = 保管庫.アップロードする(&識別子, &検証済み)?;
    assert_eq!(初回結果, アップロード結果::転送済み);

    let 転送後状態 = 保管庫.存在を確認する(&識別子, バイト数)?;
    assert_eq!(転送後状態, オブジェクト状態::存在);

    let 再アップロード結果 = 保管庫.アップロードする(&識別子, &検証済み)?;
    assert_eq!(再アップロード結果, アップロード結果::既存);

    let ダウンロード先一時ディレクトリ = 一時ディレクトリ::生成する(作業ディレクトリ.path());
    let ダウンロード先 = ダウンロード先一時ディレクトリ.固有の一時ファイルパスを払い出す();
    保管庫.ダウンロードする(&識別子, バイト数, &ダウンロード先)?;

    let ダウンロードした内容 = std::fs::read(ダウンロード先.パス())?;
    assert_eq!(ダウンロードした内容, 内容);

    eprintln!(
        "実rcloneのlocal backendでの往復に成功しました: 実行ファイル={}, 保管先ルート={}",
        実行ファイルパス.display(),
        保管先ルート.path().display()
    );

    Ok(())
}
