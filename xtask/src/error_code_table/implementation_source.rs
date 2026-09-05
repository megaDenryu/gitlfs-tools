//! `crates/gitlfs-tools-protocol/src/error_code.rs`が定める番号割り当ての読み取り。
//!
//! xtaskは外部依存も他クレートへの依存も持たないため、実装の一覧はソースを文字列として
//! 読み取って得る。読み取る形は`エラーコード::値()`のmatch腕である`Self::<名前> => <番号>,`
//! であり、Rustの構文としてこの綴りしか取り得ない。書式が変わって1件も読み取れなくなった
//! ときは、黙って合格せずエラーで止める。

use std::path::Path;

use crate::error_code_table::error_code_number::エラーコード番号;

pub struct 実装のエラーコード {
    pub 番号: エラーコード番号,
    pub 名前: String,
}

pub fn 実装の一覧を読み取る(実装ソース: &Path) -> Result<Vec<実装のエラーコード>, String> {
    let 内容 = std::fs::read_to_string(実装ソース)
        .map_err(|失敗| format!("{}を読み込めなかった: {失敗}", 実装ソース.display()))?;

    let 一覧: Vec<実装のエラーコード> = 内容.lines().filter_map(割り当ての1行を読み取る).collect();

    if 一覧.is_empty() {
        return Err(format!(
            "{}からエラーコードの割り当てを1件も読み取れなかった。`Self::<名前> => <番号>,`の書式が変わった可能性がある",
            実装ソース.display()
        ));
    }
    Ok(一覧)
}

fn 割り当ての1行を読み取る(行: &str) -> Option<実装のエラーコード> {
    let 割り当て = 行.trim().strip_prefix("Self::")?.strip_suffix(',')?;
    let (名前, 番号欄) = 割り当て.split_once("=>")?;
    Some(実装のエラーコード {
        番号: エラーコード番号::生成する(番号欄.trim().parse::<u32>().ok()?),
        名前: 名前.trim().to_owned(),
    })
}
