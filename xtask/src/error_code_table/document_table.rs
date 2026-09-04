//! `_doc/利用/トラブルシューティング.md`の「## エラーコード対応表」節にある表の読み取り。
//!
//! 読み取る範囲をこの節に限るため、文書の他の表に数字で始まる行があっても混ざらない。
//! 表の書式は`| コード | 意味 | 利用者が取るべき対処 |`であり、1列目が数値として読める行
//! だけを登録として扱う。見出し行と区切り線は1列目が数値にならないため自然に外れる。

use std::path::Path;

use crate::error_code_table::error_code_number::エラーコード番号;

const 対応表の見出し: &str = "## エラーコード対応表";

pub struct 文書のエラーコード {
    pub 番号: エラーコード番号,
    pub 意味: String,
}

pub fn 文書の一覧を読み取る(対応表文書: &Path) -> Result<Vec<文書のエラーコード>, String> {
    let 内容 = std::fs::read_to_string(対応表文書)
        .map_err(|失敗| format!("{}を読み込めなかった: {失敗}", 対応表文書.display()))?;

    let 節 = 対応表の節を切り出す(&内容).ok_or_else(|| {
        format!("{}に「{対応表の見出し}」の節が見つからなかった", 対応表文書.display())
    })?;
    let 一覧: Vec<文書のエラーコード> = 節.lines().filter_map(表の1行を読み取る).collect();

    if 一覧.is_empty() {
        return Err(format!(
            "{}の「{対応表の見出し}」節から表の行を1件も読み取れなかった。表の書式が変わった可能性がある",
            対応表文書.display()
        ));
    }
    Ok(一覧)
}

fn 対応表の節を切り出す(内容: &str) -> Option<&str> {
    let 見出しより後ろ = 内容.split_once(対応表の見出し)?.1;
    Some(match 見出しより後ろ.split_once("\n## ") {
        Some((節, _)) => 節,
        None => 見出しより後ろ,
    })
}

fn 表の1行を読み取る(行: &str) -> Option<文書のエラーコード> {
    let 行 = 行.trim();
    if !行.starts_with('|') {
        return None;
    }
    let セル: Vec<&str> = 行.trim_matches('|').split('|').map(str::trim).collect();
    let [コード欄, 意味欄, ..] = セル.as_slice() else {
        return None;
    };
    Some(文書のエラーコード {
        番号: エラーコード番号::生成する(コード欄.parse::<u32>().ok()?),
        意味: (*意味欄).to_owned(),
    })
}
