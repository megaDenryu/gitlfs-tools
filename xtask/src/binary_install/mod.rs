//! ビルド済みの実行ファイルを、`target/`の外の安定した場所へ配置するコマンド。
//!
//! 配置したパスを標準出力へ出す。利用者はそのパスから`install`を実行するか、そのパスを
//! `install --path`へ渡すことで、Git設定へ安定した絶対パスを登録できる。
//! PATHの書き換えは行わない。環境変数の変更は他のソフトへ影響し、元へ戻す手段を
//! 利用者が持たないためである。

mod install_directory;
mod release_build;

use std::path::Path;

use crate::command_registry::サブコマンド;
use install_directory::{実行ファイル配置ディレクトリ, 配置結果};
use release_build::{リリースビルド成果物, 実行ファイル名を組み立てる};

pub struct 実行ファイル配置コマンド;

impl サブコマンド for 実行ファイル配置コマンド {
    fn 名前(&self) -> &'static str {
        "install-binary"
    }

    fn 説明(&self) -> &'static str {
        "releaseビルドし、Cargoのbinディレクトリへ実行ファイルを配置する"
    }

    fn 実行する(&self, _引数: &[String]) -> Result<(), String> {
        let 成果物 = リリースビルド成果物::ビルドして解決する()?;
        let 配置ディレクトリ = 実行ファイル配置ディレクトリ::環境変数から解決する()?;
        let 実行ファイル名 = 実行ファイル名を組み立てる();

        let 結果 = 配置ディレクトリ.配置する(成果物.パス(), &実行ファイル名)?;
        let 配置先 = 配置ディレクトリ.配置先のパス(&実行ファイル名);

        配置結果を報告する(&結果, &配置先);
        パスの通し方を案内する(&配置ディレクトリ);
        println!("{}", 配置先.display());
        Ok(())
    }
}

fn 配置結果を報告する(結果: &配置結果, 配置先: &Path) {
    match 結果 {
        配置結果::新規配置 => eprintln!("実行ファイルを新しく配置した: {}", 配置先.display()),
        配置結果::上書き配置 => eprintln!("既存の実行ファイルを上書きした: {}", 配置先.display()),
        配置結果::既存を退避して配置 { 退避先 } => {
            eprintln!("既存の実行ファイルが使用中だったため退避してから配置した: {}", 配置先.display());
            eprintln!("退避したファイルは削除できなかった。使用中のプロセスを終えた後に手で削除する: {}", 退避先.display());
        }
    }
}

fn パスの通し方を案内する(配置ディレクトリ: &実行ファイル配置ディレクトリ) {
    let ディレクトリ = 配置ディレクトリ.ディレクトリのパス().display();
    if 配置ディレクトリ.パスが通っているか() {
        eprintln!("配置先ディレクトリはPATHに含まれている: {ディレクトリ}");
        eprintln!("次の手順: 対象リポジトリで git-lfs-rclone-storage install を実行する");
        return;
    }
    eprintln!("配置先ディレクトリはPATHに含まれていない: {ディレクトリ}");
    eprintln!("このディレクトリをPATHへ加えると、名前だけで起動できる。加えるかどうかは利用者が決める");
    eprintln!("加えない場合は、対象リポジトリで次を実行する");
    eprintln!("  git-lfs-rclone-storage install --path <上記の配置先>");
}
