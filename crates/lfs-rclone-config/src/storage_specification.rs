//! PC設定の`storage`キーが選ぶ保管先の種類と、その種類だけが必要とする設定値。
//!
//! `storage`を省略した設定ファイルは従来どおりrclone子プロセス方式として扱う（既存のPC設定を
//! 壊さない）。種類ごとに必要なキーが違うため、種類の判別と同時に、必要なキーの欠落と
//! 使わないキーの指定の両方を拒否する。使わないキーを黙って無視すると、`storage`の書き換え
//! だけで方式を切り替えたつもりの設定が、実際には片方の設定を残したまま動く。

use std::time::Duration;

use lfs_rclone_domain::{Rclone実行ファイルの場所, Rcloneリモート名, 転送タイムアウト};

use crate::config_error::設定エラー;
use crate::local_storage_root::ローカルファイルシステム上の保管先ルート;
use crate::pc_config_toml::PCプロファイルTOML表現;

const RCLONE子プロセス方式の名前: &str = "rclone";
const ローカルディレクトリ方式の名前: &str = "local";
const 受理できる保管先の種類の一覧: &str = "rclone, local";

/// 解決済みプロファイルの保管先。種類によって必要な設定値が違うため、判別共用体の各枝が
/// その種類だけが使う値を持つ（文字列のまま上位層へ運ばない）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 保管先の指定 {
    Rclone子プロセス { リモート名: Rcloneリモート名, 実行ファイル: Rclone実行ファイルの場所, 転送タイムアウト: 転送タイムアウト },
    ローカルディレクトリ { ルートディレクトリ: ローカルファイルシステム上の保管先ルート },
}

impl 保管先の指定 {
    /// TOML表現を参照で受け取り、必要な値だけを複製して組み立てる。呼び出し側の署名を
    /// プリミティブの列にしないことを優先しており、複製されるのは起動時に1度、短い文字列
    /// 2つだけである。
    pub(crate) fn 表現から生成する(表現: &PCプロファイルTOML表現) -> Result<Self, 設定エラー> {
        match 表現.storage.as_deref().unwrap_or(RCLONE子プロセス方式の名前) {
            RCLONE子プロセス方式の名前 => rclone子プロセス方式として生成する(表現),
            ローカルディレクトリ方式の名前 => ローカルディレクトリ方式として生成する(表現),
            受理できない値 => Err(設定エラー::未対応の保管先の種類 {
                受信した値: 受理できない値.to_owned(),
                受理できる値: 受理できる保管先の種類の一覧.to_owned(),
            }),
        }
    }
}

fn rclone子プロセス方式として生成する(表現: &PCプロファイルTOML表現) -> Result<保管先の指定, 設定エラー> {
    let リモート名文字列 = 表現.rclone_remote.clone().ok_or_else(|| 必須キー不足にする("rclone_remote"))?;
    let リモート名 = Rcloneリモート名::生成する(リモート名文字列)
        .map_err(|エラー| 設定エラー::解析失敗 { 説明: エラー.to_string() })?;
    let 秒数 = 表現.transfer_timeout_seconds.ok_or_else(|| 必須キー不足にする("transfer_timeout_seconds"))?;

    Ok(保管先の指定::Rclone子プロセス {
        リモート名,
        実行ファイル: 実行ファイル指定文字列から生成する(表現.rclone_executable.clone()),
        転送タイムアウト: 転送タイムアウト::生成する(Duration::from_secs(秒数)),
    })
}

fn ローカルディレクトリ方式として生成する(表現: &PCプロファイルTOML表現) -> Result<保管先の指定, 設定エラー> {
    子プロセス方式だけが使うキーが無いことを確かめる(表現)?;

    Ok(保管先の指定::ローカルディレクトリ {
        ルートディレクトリ: ローカルファイルシステム上の保管先ルート::生成する(&表現.base_path),
    })
}

/// ローカルディレクトリ方式では子プロセスを起動しないため、rcloneの3キーは意味を持たない。
fn 子プロセス方式だけが使うキーが無いことを確かめる(表現: &PCプロファイルTOML表現) -> Result<(), 設定エラー> {
    let 指定済みのキー = [
        ("rclone_remote", 表現.rclone_remote.is_some()),
        ("rclone_executable", 表現.rclone_executable.is_some()),
        ("transfer_timeout_seconds", 表現.transfer_timeout_seconds.is_some()),
    ];
    match 指定済みのキー.into_iter().find(|(_, 指定されているか)| *指定されているか) {
        Some((キー名, _)) => Err(設定エラー::この保管先の種類では使わないキー {
            キー名: キー名.to_owned(),
            保管先の種類: ローカルディレクトリ方式の名前.to_owned(),
        }),
        None => Ok(()),
    }
}

/// TOMLの`rclone_executable`（省略可能な文字列）から`Rclone実行ファイルの場所`を組み立てる。
/// 「未指定ならPATH解決に委ねる」というTOML固有の解釈を扱うため、domain層でなくこの層に置く。
fn 実行ファイル指定文字列から生成する(値: Option<String>) -> Rclone実行ファイルの場所 {
    match 値 {
        Some(パス文字列) => Rclone実行ファイルの場所::指定パスから生成する(パス文字列),
        None => Rclone実行ファイルの場所::解決を環境変数に委ねる(),
    }
}

fn 必須キー不足にする(キー名: &str) -> 設定エラー {
    設定エラー::プロファイルの必須キー不足 { キー名: キー名.to_owned() }
}
