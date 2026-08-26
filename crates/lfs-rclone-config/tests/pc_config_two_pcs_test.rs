//! 同じ論理プロファイル名が、PCごとに異なるrcloneリモート名へ解決されることのテスト
//! （Issue #4完了条件「PC AとPC Bで同じ論理profile名から異なるlocal remote名へ解決できる」）。

use std::io;

use lfs_rclone_config::PC設定の場所;
use lfs_rclone_domain::プロファイル名;

fn pc設定ディレクトリを作る(内容: &str) -> io::Result<tempfile::TempDir> {
    let ディレクトリ = tempfile::tempdir()?;
    std::fs::write(ディレクトリ.path().join("config.toml"), 内容)?;
    Ok(ディレクトリ)
}

#[test]
fn 同じ論理プロファイル名がpcごとに異なるリモート名へ解決される() -> Result<(), Box<dyn std::error::Error>> {
    let pcaの内容 = "schema_version = 1\n\
                     [profiles.personal-large-assets]\n\
                     rclone_remote = \"pc-a-remote\"\n\
                     base_path = \"git-lfs-rclone-storage\"\n\
                     temp_directory = \"D:/tmp-a\"\n\
                     transfer_timeout_seconds = 3600\n";
    let pcbの内容 = "schema_version = 1\n\
                     [profiles.personal-large-assets]\n\
                     rclone_remote = \"pc-b-remote\"\n\
                     base_path = \"git-lfs-rclone-storage\"\n\
                     temp_directory = \"/home/user/tmp-b\"\n\
                     transfer_timeout_seconds = 1800\n";

    let pcaディレクトリ = pc設定ディレクトリを作る(pcaの内容)?;
    let pcbディレクトリ = pc設定ディレクトリを作る(pcbの内容)?;

    let pca設定 = PC設定の場所::ディレクトリを指定して生成する(pcaディレクトリ.path()).読み込む()?;
    let pcb設定 = PC設定の場所::ディレクトリを指定して生成する(pcbディレクトリ.path()).読み込む()?;

    let プロファイル名 = プロファイル名::生成する("personal-large-assets")?;

    let pcaのプロファイル = pca設定.プロファイルを解決する(&プロファイル名)?;
    let pcbのプロファイル = pcb設定.プロファイルを解決する(&プロファイル名)?;

    assert_eq!(pcaのプロファイル.rcloneリモート().文字列表現(), "pc-a-remote");
    assert_eq!(pcbのプロファイル.rcloneリモート().文字列表現(), "pc-b-remote");
    assert_ne!(
        pcaのプロファイル.rcloneリモート().文字列表現(),
        pcbのプロファイル.rcloneリモート().文字列表現()
    );
    Ok(())
}
