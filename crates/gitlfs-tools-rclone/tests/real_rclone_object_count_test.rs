//! 実rcloneプロセスとの結合テスト。`保管しているオブジェクトの総数を数える`が、実rcloneの
//! local backendで置き場未作成（終了コード3）を0件として扱い、アップロード後に件数を返し、
//! 一時アップロード領域を数に入れないことを確かめる。
//!
//! 実行ファイルの解決と読み飛ばしの規律は`real_rclone_local_backend_test.rs`と同じである。

mod common;

use std::io::Write;
use std::path::Path;
use std::time::Duration;

use gitlfs_tools_domain::{
    オブジェクト識別子, 保管先基底パス, 期待バイト数, Rclone実行ファイルの場所, Rcloneリモート名, 検証前のローカルファイル, 転送タイムアウト,
};
use gitlfs_tools_rclone::Rclone保管庫;
use gitlfs_tools_storage_port::オブジェクト保管庫;
use sha2::{Digest, Sha256};

fn ドライブとパスへ分ける(絶対パス: &Path) -> Result<(String, String), Box<dyn std::error::Error>> {
    let 文字列 = 絶対パス.to_str().ok_or("一時ディレクトリのパスがUTF-8ではありません")?;
    let 正規化 = 文字列.replace('\\', "/");
    let (ドライブ, 残り) = 正規化.split_once(':').ok_or("絶対パスにドライブ文字がありません")?;
    Ok((ドライブ.to_owned(), 残り.to_owned()))
}

fn アップロードする(保管庫: &Rclone保管庫, 内容: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let 十六進文字列: String = Sha256::digest(内容).iter().map(|バイト| format!("{バイト:02x}")).collect();
    let 識別子 = オブジェクト識別子::生成する(&十六進文字列)?;
    let バイト数 = 期待バイト数::生成する(u64::try_from(内容.len())?);
    let mut ファイル = tempfile::NamedTempFile::new()?;
    ファイル.write_all(内容)?;
    let 検証済み = 検証前のローカルファイル::生成する(ファイル.path()).検証する(&識別子, バイト数)?;
    保管庫.アップロードする(&識別子, &検証済み)?;
    Ok(())
}

#[test]
fn 実rcloneのlocal_backendで保管先のオブジェクト数を数える() -> Result<(), Box<dyn std::error::Error>> {
    let 実行ファイル = match common::rclone_executable::実行ファイルを解決する()? {
        common::rclone_executable::実行ファイル解決::明示された場所(パス) => Rclone実行ファイルの場所::指定パスから生成する(パス),
        common::rclone_executable::実行ファイル解決::PATH解決 => Rclone実行ファイルの場所::解決を環境変数に委ねる(),
        common::rclone_executable::実行ファイル解決::読み飛ばす => {
            eprintln!("GITLFS_TOOLS_SKIP_INTEGRATION が設定されているため、実rcloneとの結合テストを読み飛ばします。");
            return Ok(());
        }
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = ドライブとパスへ分ける(保管先ルート.path())?;
    let 保管庫 = Rclone保管庫::生成する(
        実行ファイル,
        Rcloneリモート名::生成する(ドライブ)?,
        保管先基底パス::生成する(残り)?,
        転送タイムアウト::生成する(Duration::from_secs(30)),
    );

    assert_eq!(保管庫.保管しているオブジェクトの総数を数える()?.件数(), 0, "置き場が未作成なら0件として扱うべき");

    アップロードする(&保管庫, b"real rclone object count payload one")?;
    アップロードする(&保管庫, b"real rclone object count payload two")?;

    assert_eq!(保管庫.保管しているオブジェクトの総数を数える()?.件数(), 2);

    let 一時領域 = 保管先ルート.path().join("lfs").join("tmp");
    std::fs::create_dir_all(&一時領域)?;
    std::fs::write(一時領域.join("転送途中の残骸"), b"leftover")?;

    assert_eq!(保管庫.保管しているオブジェクトの総数を数える()?.件数(), 2, "一時アップロード領域を数に入れてはならない");
    Ok(())
}
