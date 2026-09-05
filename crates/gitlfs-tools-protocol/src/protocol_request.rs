//! stdinの1行分のJSONを、日本語ドメインの`転送プロトコル要求`へ変換する。

use crate::incoming_event_json::受信イベントJSON;
use crate::init_request::初期化要求;
use crate::protocol_parse_error::プロトコル解析エラー;
use crate::transfer_operation_kind::転送操作種別;

/// 1行分のJSONから読み取った要求。`アップロード`・`ダウンロード`は生のフィールドを保持し、
/// 識別子の形式検証は行わない。`oid`の値がどれだけ不正でも、それを`complete`失敗の宛先
/// として使えるようにするためである（呼び出し側`protocol_session.rs`の責務）。
pub(crate) enum 転送プロトコル要求 {
    初期化(初期化要求),
    アップロード { oid: String, size: u64, path: String },
    ダウンロード { oid: String, size: u64 },
    終了,
}

/// 1行分の文字列を解析する。`event`が識別できないか、`upload`/`download`に`oid`が
/// 欠落している場合だけ`プロトコル解析エラー`とする。それ以外の不備（`size`・`path`の
/// 欠落、`oid`の形式不正）は`oid`付きで下流の検証へ委ね、致命的エラーにしない。
pub(crate) fn 行から要求を解析する(行: &str) -> Result<転送プロトコル要求, プロトコル解析エラー> {
    let 受信データ: 受信イベントJSON =
        serde_json::from_str(行).map_err(|エラー| プロトコル解析エラー::JSON解析失敗 { 説明: エラー.to_string() })?;

    match 受信データ.event.as_str() {
        "init" => {
            let 操作種別 = 転送操作種別::文字列から生成する(受信データ.operation.as_deref())?;
            Ok(転送プロトコル要求::初期化(初期化要求::生成する(操作種別)))
        }
        "upload" => {
            let oid = oidを取り出す(受信データ.oid, "upload")?;
            Ok(転送プロトコル要求::アップロード { oid, size: 受信データ.size.unwrap_or(0), path: 受信データ.path.unwrap_or_default() })
        }
        "download" => {
            let oid = oidを取り出す(受信データ.oid, "download")?;
            Ok(転送プロトコル要求::ダウンロード { oid, size: 受信データ.size.unwrap_or(0) })
        }
        "terminate" => Ok(転送プロトコル要求::終了),
        他 => Err(プロトコル解析エラー::未知のevent { 値: 他.to_owned() }),
    }
}

fn oidを取り出す(oid: Option<String>, イベント名: &str) -> Result<String, プロトコル解析エラー> {
    oid.ok_or_else(|| プロトコル解析エラー::必須フィールド欠落または不正 {
        説明: format!("{イベント名}にoidがありません"),
    })
}
