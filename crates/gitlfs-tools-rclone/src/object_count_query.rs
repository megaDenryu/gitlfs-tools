//! 保管先のオブジェクト置き場に在るオブジェクトの総数を`size --json`で問い合わせる外部境界。
//!
//! `lsjson -R`でも数えられるが、そちらは全オブジェクトの一覧をJSONで受け取るため、件数が
//! 増えるほど出力が太る。`size --json`は`{"count":N,"bytes":M,"sizeless":0}`の1行だけを
//! 返すため、件数によらず出力の大きさが変わらない。
//!
//! 注意: 置き場のディレクトリがまだ一度も作られていない場合、local backendは終了コード3
//! （directory not found）で失敗する。これは「まだ1件もアップロードしていない」状態であり
//! 失敗ではないため、0件として扱う（`existence_query.rs`が未存在の判定で同じ扱いをする）。
//! rclone v1.75.0での実測に基づく。

use std::ffi::OsString;

use gitlfs_tools_domain::{保管エラー, 保管先オブジェクトパス, Rcloneリモート名};
use gitlfs_tools_storage_port::保管オブジェクト総数;
use serde::Deserialize;

use crate::rclone_execution_error::Rclone実行エラー;
use crate::rclone_operation::Rclone操作;
use crate::rclone_process_runner::Rcloneプロセス実行器;

const ディレクトリ未存在の終了コード: i32 = 3;

#[derive(Debug, Deserialize)]
struct Size集計 {
    #[serde(rename = "count")]
    件数: u64,
}

pub(crate) fn 保管先のオブジェクト総数を問い合わせる(
    実行器: &Rcloneプロセス実行器,
    リモート名: &Rcloneリモート名,
    オブジェクト置き場: &保管先オブジェクトパス,
) -> Result<保管オブジェクト総数, 保管エラー> {
    let 対象 = format!("{}:{}", リモート名.文字列表現(), オブジェクト置き場.文字列表現());
    let 引数 = vec![
        OsString::from("size"),
        OsString::from("--json"),
        OsString::from("-q"),
        OsString::from("--stats"),
        OsString::from("0"),
        OsString::from(対象),
    ];

    let 標準出力 = match 実行器.実行する(Rclone操作::オブジェクト数の集計, &引数) {
        Ok(標準出力) => 標準出力,
        Err(Rclone実行エラー::非0終了 { 終了コード: Some(ディレクトリ未存在の終了コード), .. }) => {
            return Ok(保管オブジェクト総数::生成する(0));
        }
        Err(エラー) => return Err(エラー.into()),
    };

    let 集計: Size集計 = serde_json::from_slice(&標準出力).map_err(|エラー| 保管エラー::子プロセス {
        説明: format!("sizeの出力を解析できませんでした: {エラー}"),
    })?;

    Ok(保管オブジェクト総数::生成する(集計.件数))
}
