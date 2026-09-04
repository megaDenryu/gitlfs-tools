//! download: 成功、未存在、整合性失敗、失敗後の継続、進捗の最終値、terminate後に失敗分の
//! 一時ファイルが残らないことを確かめる。実rcloneが必要なため、PATHまたは
//! `LFS_RCLONE_TEST_EXECUTABLE`から実行ファイルを解決する。解決できなければ、
//! `LFS_RCLONE_SKIP_INTEGRATION`が設定されている場合に限り読み飛ばし、それ以外は失敗させる
//! （`common::rclone_executable`参照）。

mod common;

use std::path::Path;

use serde_json::json;

#[test]
fn downloadの成功未存在整合性失敗と失敗後の継続を確かめる() -> Result<(), Box<dyn std::error::Error>> {
    let rclone実行ファイル = match common::rclone_executable::実行ファイルを解決する()? {
        common::rclone_executable::実行ファイル解決::明示された場所(パス) => Some(パス),
        common::rclone_executable::実行ファイル解決::PATH解決 => None,
        common::rclone_executable::実行ファイル解決::読み飛ばす => {
            eprintln!("LFS_RCLONE_SKIP_INTEGRATION が設定されているため、結合テストを読み飛ばします。");
            return Ok(());
        }
    };

    let 保管先ルート = tempfile::tempdir()?;
    let (ドライブ, 残り) = common::fixtures::ドライブとパスへ分ける(保管先ルート.path())?;
    let 一時ディレクトリ = tempfile::tempdir()?;
    let 作業ツリー = common::fixtures::プロジェクト作業ツリーを作る("download-roundtrip")?;
    let pc設定 = common::fixtures::pc設定ディレクトリを作る(
        "download-roundtrip",
        &ドライブ,
        &残り,
        一時ディレクトリ.path(),
        rclone実行ファイル.as_deref(),
    )?;
    let 内容: &[u8] = b"download roundtrip test payload";
    let (アップロード元, oid, size) = common::payload::アップロード元を作る(作業ツリー.path(), "payload.bin", 内容)?;

    let mut プロセス =
        common::process::プロトコルテストプロセス::起動する(Path::new(common::fixtures::実行ファイルのパス), 作業ツリー.path(), pc設定.path())?;
    プロセス.一行送る(&json!({"event": "init", "operation": "download", "remote": "origin"}))?;
    assert_eq!(プロセス.一行受け取る()?, json!({}));

    // 事前にアップロードしてストレージへ実体を置く。
    プロセス.一行送る(&json!({"event": "upload", "oid": oid, "size": size, "path": アップロード元, "action": null}))?;
    プロセス.一行受け取る()?;
    プロセス.一行受け取る()?;

    ダウンロードして成功を確かめる(&mut プロセス, &oid, size, 内容, 作業ツリー.path())?;

    let 未存在oid = "f".repeat(64);
    プロセス.一行送る(&json!({"event": "download", "oid": 未存在oid, "size": 1, "action": null}))?;
    let 未存在応答 = プロセス.一行受け取る()?;
    assert_eq!(未存在応答["event"], "complete");
    assert!(未存在応答.get("error").is_some());

    プロセス.一行送る(&json!({"event": "download", "oid": oid, "size": size + 1, "action": null}))?;
    let 整合性失敗応答 = プロセス.一行受け取る()?;
    assert!(整合性失敗応答.get("error").is_some(), "サイズ偽装は整合性エラーになるべき: {整合性失敗応答:?}");

    ダウンロードして成功を確かめる(&mut プロセス, &oid, size, 内容, 作業ツリー.path())?; // 失敗後も継続できる

    プロセス.一行送る(&json!({"event": "terminate"}))?;
    let (終了状態, _) = プロセス.終了を待って後始末する()?;
    assert!(終了状態.success());

    let 一時ファイル置き場 = common::fixtures::ダウンロード一時ディレクトリのパス(作業ツリー.path());
    let 残存ファイル数 = std::fs::read_dir(&一時ファイル置き場)?.count();
    assert_eq!(残存ファイル数, 2, "成功した2件のdownload先だけが残るべき(失敗分は削除済み)");
    assert_eq!(
        std::fs::read_dir(一時ディレクトリ.path())?.count(),
        0,
        "PC設定のtemp_directoryはもう使われないため、agentは何も置いてはならない"
    );
    Ok(())
}

fn ダウンロードして成功を確かめる(
    プロセス: &mut common::process::プロトコルテストプロセス,
    oid: &str,
    size: u64,
    期待する内容: &[u8],
    作業ツリー: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    プロセス.一行送る(&json!({"event": "download", "oid": oid, "size": size, "action": null}))?;
    let 進捗 = プロセス.一行受け取る()?;
    assert_eq!(進捗["bytesSoFar"], size);
    let 完了 = プロセス.一行受け取る()?;
    assert_eq!(完了["event"], "complete");
    assert_eq!(完了["oid"], oid);
    let 保存先 = 完了["path"].as_str().ok_or("pathが文字列ではありません")?;
    assert_eq!(std::fs::read(保存先)?, 期待する内容);
    一時ファイルがリポジトリと同じボリュームにあることを確かめる(Path::new(保存先), 作業ツリー)
}

/// Git LFSは`complete`で受け取ったファイルをリポジトリの`.git/lfs/objects/`へ`rename`で
/// 移す。ボリュームをまたぐ`rename`は失敗するため、一時ファイルはリポジトリの`.git`の
/// 下に無ければならない。
fn 一時ファイルがリポジトリと同じボリュームにあることを確かめる(保存先: &Path, 作業ツリー: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let 置き場 = common::fixtures::ダウンロード一時ディレクトリのパス(作業ツリー);
    let 実際の親 = 保存先.parent().ok_or("一時ファイルに親ディレクトリがありません")?;
    // 短縮名(8.3形式)と区切り文字の違いを吸収するため、実体を解決してから比べる。
    if std::fs::canonicalize(実際の親)? == std::fs::canonicalize(&置き場)? {
        Ok(())
    } else {
        Err(format!("一時ファイルがリポジトリの外にある: {} (期待した置き場: {})", 保存先.display(), 置き場.display()).into())
    }
}
