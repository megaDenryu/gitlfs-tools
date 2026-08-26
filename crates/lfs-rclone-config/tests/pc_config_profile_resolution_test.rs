//! PC設定からのプロファイル解決(正常解決・未定義プロファイル・rclone_executableの
//! 省略/明示の区別)のテスト。

mod common;

use std::path::PathBuf;

use lfs_rclone_config::{PC設定の場所, 設定エラー};
use lfs_rclone_domain::{Rclone実行ファイルの場所, プロファイル名};

use common::pc設定ディレクトリを作る;

#[test]
fn 正常なpc設定を解析しプロファイルを解決する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.personal-large-assets]\n\
                rclone_remote = \"mega-assets\"\n\
                base_path = \"git-lfs-rclone-storage\"\n\
                temp_directory = \"D:/large-assets-tmp\"\n\
                transfer_timeout_seconds = 3600\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;

    let pc設定 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む()?;
    let プロファイル名 = プロファイル名::生成する("personal-large-assets")?;
    let プロファイル = pc設定.プロファイルを解決する(&プロファイル名)?;

    assert_eq!(プロファイル.rcloneリモート().文字列表現(), "mega-assets");
    // `保管先基底パス`は生の文字列アクセサを公開しない値型のため、一時アップロード先を
    // 払い出して先頭が期待どおりの基底パスになっていることで間接的に検証する。
    let 一時アップロード先 = プロファイル.基底パス().一時アップロード先を払い出す();
    assert!(一時アップロード先.文字列表現().starts_with("git-lfs-rclone-storage/lfs/tmp/"));
    Ok(())
}

#[test]
fn 未定義プロファイルを他の失敗と区別できる() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.personal-large-assets]\n\
                rclone_remote = \"mega-assets\"\n\
                base_path = \"git-lfs-rclone-storage\"\n\
                temp_directory = \"D:/large-assets-tmp\"\n\
                transfer_timeout_seconds = 3600\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;
    let pc設定 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む()?;
    let 未定義のプロファイル名 = プロファイル名::生成する("does-not-exist")?;

    let 結果 = pc設定.プロファイルを解決する(&未定義のプロファイル名);

    assert!(matches!(
        結果,
        Err(設定エラー::未定義プロファイル { ref プロファイル名 }) if プロファイル名.文字列表現() == "does-not-exist"
    ));
    Ok(())
}

#[test]
fn rclone_executableの省略と明示指定を区別できる() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.without-executable]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                transfer_timeout_seconds = 1\n\
                [profiles.with-executable]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                transfer_timeout_seconds = 1\n\
                rclone_executable = \"C:/tools/rclone.exe\"\n";
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;
    let pc設定 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む()?;

    let 省略プロファイル名 = プロファイル名::生成する("without-executable")?;
    let 明示プロファイル名 = プロファイル名::生成する("with-executable")?;

    let 省略時 = pc設定.プロファイルを解決する(&省略プロファイル名)?;
    let 明示時 = pc設定.プロファイルを解決する(&明示プロファイル名)?;

    assert_eq!(省略時.rclone実行ファイル(), &Rclone実行ファイルの場所::PATH上の実行ファイル);
    assert_eq!(
        明示時.rclone実行ファイル(),
        &Rclone実行ファイルの場所::明示された場所(PathBuf::from("C:/tools/rclone.exe"))
    );
    Ok(())
}
