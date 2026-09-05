//! PC設定の`storage`キーによる保管先の種類の解決（省略時の既定・ローカルディレクトリ方式・
//! 未知の値の拒否・種類ごとに必要なキーと使わないキーの検査）のテスト。

mod common;

use gitlfs_tools_config::{PC設定の場所, PCプロファイル, 保管先の指定, 設定エラー};
use gitlfs_tools_domain::プロファイル名;

use common::pc設定ディレクトリを作る;

fn プロファイルを解決する(内容: &str) -> Result<Result<PCプロファイル, 設定エラー>, Box<dyn std::error::Error>> {
    let ディレクトリ = pc設定ディレクトリを作る(内容)?;
    let 読み込み結果 = PC設定の場所::ディレクトリを指定して生成する(ディレクトリ.path()).読み込む();
    let pc設定 = match 読み込み結果 {
        Ok(pc設定) => pc設定,
        Err(エラー) => return Ok(Err(エラー)),
    };
    let プロファイル名 = プロファイル名::生成する("x")?;
    Ok(pc設定.プロファイルを解決する(&プロファイル名).cloned())
}

#[test]
fn storageを省略すればrclone子プロセス方式として解決する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                rclone_remote = \"r\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                transfer_timeout_seconds = 1\n";

    let プロファイル = プロファイルを解決する(内容)??;

    assert!(matches!(プロファイル.保管先(), 保管先の指定::Rclone子プロセス { .. }));
    Ok(())
}

#[test]
fn storageにlocalを指定すればローカルディレクトリ方式として解決する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                storage = \"local\"\n\
                base_path = \"G:/mounted-drive/gitlfs-tools\"\n\
                temp_directory = \"t\"\n";

    let プロファイル = プロファイルを解決する(内容)??;

    let 保管先の指定::ローカルディレクトリ { ルートディレクトリ } = プロファイル.保管先() else {
        return Err("ローカルディレクトリ方式として解決されるべき".into());
    };
    assert_eq!(ルートディレクトリ.パス(), std::path::Path::new("G:/mounted-drive/gitlfs-tools"));
    Ok(())
}

#[test]
fn 未知のstorageの値を受理できる値とともに拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                storage = \"s3\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n";

    let 結果 = プロファイルを解決する(内容)?;

    assert!(matches!(
        結果,
        Err(設定エラー::未対応の保管先の種類 { ref 受信した値, ref 受理できる値 })
            if 受信した値 == "s3" && 受理できる値 == "rclone, local"
    ));
    Ok(())
}

#[test]
fn ローカルディレクトリ方式でrclone用のキーを指定すれば拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                storage = \"local\"\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                rclone_remote = \"r\"\n";

    let 結果 = プロファイルを解決する(内容)?;

    assert!(matches!(結果, Err(設定エラー::この保管先の種類では使わないキー { ref キー名, .. }) if キー名 == "rclone_remote"));
    Ok(())
}

#[test]
fn rclone子プロセス方式で必須キーが欠けていれば拒否する() -> Result<(), Box<dyn std::error::Error>> {
    let 内容 = "schema_version = 1\n\
                [profiles.x]\n\
                base_path = \"b\"\n\
                temp_directory = \"t\"\n\
                transfer_timeout_seconds = 1\n";

    let 結果 = プロファイルを解決する(内容)?;

    assert!(matches!(結果, Err(設定エラー::プロファイルの必須キー不足 { ref キー名 }) if キー名 == "rclone_remote"));
    Ok(())
}
