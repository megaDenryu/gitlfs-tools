//! 最終保存先の1オブジェクトの存在とバイト数をlsjsonで問い合わせる外部境界。
//!
//! 注意: `lsjson`は`--stat`を付けない形で使う。ドキュメントは見つからない場合にエラーに
//! なると書いているが、実装のほとんどの経路は終了コード0のまま標準出力へ`null`を書いて
//! 終わる（`--stat`のときだけ）。`--stat`を付けない本経路は常にJSON配列を返し、存在すれば
//! 要素1個・存在しなければ空配列`[]`になるため、終了コードに頼らず出力の中身だけで
//! 判定できる（scratchpad/rclone-subcommands.md 1節の実測に基づく）。
//!
//! 注意: 上記は「対象の親ディレクトリまでは存在する」場合の話である。実rcloneのlocal
//! backendで実測したところ、`<基底パス>/lfs/objects/sha256/<ab>/<cd>/`という中間
//! ディレクトリ自体がまだ一度も作られていない場合（オブジェクト未存在の典型的な初回問い合わせ）、
//! localバックエンドは空配列を返さず終了コード3（Directory not found）で失敗する。
//! これはバケット型バックエンド（s3等。空ディレクトリという概念を持たない）とlocalのような
//! 階層型バックエンドとの違いであり、scratchpad/rclone-subcommands.md 5節が転記した
//! ドキュメント本文の「except for remotes which can't have empty directories」の
//! 「except」に該当しない側である。よって終了コード3も「未存在」として扱う。

use std::ffi::OsString;

use lfs_rclone_domain::{保管エラー, 保管先オブジェクトパス, 期待バイト数, Rcloneリモート名};
use serde::Deserialize;

use crate::rclone_execution_error::Rclone実行エラー;
use crate::rclone_operation::Rclone操作;
use crate::rclone_process_runner::Rcloneプロセス実行器;

const ディレクトリ未存在の終了コード: i32 = 3;

#[derive(Debug, Deserialize)]
struct Lsjson要素 {
    #[serde(rename = "Size")]
    サイズ: u64,
}

/// 対象が存在すれば実バイト数を、存在しなければ`None`を返す。
pub(crate) fn 最終オブジェクトの存在を問い合わせる(
    実行器: &Rcloneプロセス実行器,
    リモート名: &Rcloneリモート名,
    パス: &保管先オブジェクトパス,
) -> Result<Option<期待バイト数>, 保管エラー> {
    let 対象 = format!("{}:{}", リモート名.文字列表現(), パス.文字列表現());
    let 引数 = vec![
        OsString::from("lsjson"),
        OsString::from("--files-only"),
        OsString::from("-q"),
        OsString::from("--stats"),
        OsString::from("0"),
        OsString::from(対象),
    ];

    let 標準出力 = match 実行器.実行する(Rclone操作::存在確認, &引数) {
        Ok(標準出力) => 標準出力,
        Err(Rclone実行エラー::非0終了 { 終了コード: Some(ディレクトリ未存在の終了コード), .. }) => return Ok(None),
        Err(エラー) => return Err(エラー.into()),
    };

    let 要素一覧: Vec<Lsjson要素> = serde_json::from_slice(&標準出力).map_err(|エラー| 保管エラー::子プロセス {
        説明: format!("lsjsonの出力を解析できませんでした: {エラー}"),
    })?;

    Ok(要素一覧.into_iter().next().map(|要素| 期待バイト数::生成する(要素.サイズ)))
}
