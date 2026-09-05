//! ワークスペース全体の版。`Cargo.toml`の`[workspace.package]`の`version`が唯一の出所である。
//!
//! 版を上げるのは人であり、xtaskはこの値を書き換えない。どの版を出すかは、実装の内容を
//! 見た人だけが決められる判断であり、機械が決めると意図しない版が世に出るためである。

use std::path::PathBuf;

/// `[workspace.package]`に書かれている版の綴り（`1.0.0`）。
pub struct ワークスペースの版(String);

impl ワークスペースの版 {
    pub fn リポジトリルートの設定ファイルから読む() -> Result<Self, String> {
        let パス = 設定ファイルのパスを組み立てる()?;
        let 本文 =
            std::fs::read_to_string(&パス).map_err(|失敗| format!("{}を読み取れなかった: {失敗}", パス.display()))?;
        let 版 = ワークスペースの節から版を抜き出す(&本文).ok_or_else(|| {
            format!("{}の[workspace.package]にversionが見つからない", パス.display())
        })?;
        if !数字3組の版の綴りか(&版) {
            return Err(format!("{}のversionの値'{版}'を数字3組の版として解析できない。行末コメント等を外す", パス.display()));
        }
        Ok(Self(版))
    }

    pub fn 版の文字列(&self) -> &str {
        &self.0
    }
}

fn 設定ファイルのパスを組み立てる() -> Result<PathBuf, String> {
    std::env::current_dir()
        .map(|現在地| 現在地.join("Cargo.toml"))
        .map_err(|失敗| format!("カレントディレクトリを取得できなかった: {失敗}"))
}

/// 節の見出し行で現在位置を切り替えながら、`[workspace.package]`の中の`version`だけを拾う。
/// 各クレートの`version.workspace = true`と取り違えないため、`=`が続く行だけを版とみなす。
fn ワークスペースの節から版を抜き出す(本文: &str) -> Option<String> {
    let mut 節の中か = false;
    for 行 in 本文.lines() {
        let 整えた行 = 行.trim();
        if 整えた行.starts_with('[') {
            節の中か = 整えた行 == "[workspace.package]";
            continue;
        }
        if !節の中か {
            continue;
        }
        if let Some(右辺) = 整えた行.strip_prefix("version").map(str::trim_start).and_then(|残り| 残り.strip_prefix('='))
        {
            return Some(右辺.trim().trim_matches('"').to_owned());
        }
    }
    None
}

/// `1.0.0`のように、数字だけの組が`.`で3つ並ぶ綴りだけを版とみなす。行末コメントや
/// 引用符の取り残しが混ざった値をタグ名へ運ばないための検査である。
fn 数字3組の版の綴りか(綴り: &str) -> bool {
    let 組 = 綴り.split('.').collect::<Vec<_>>();
    組.len() == 3 && 組.iter().all(|数| !数.is_empty() && 数.chars().all(|文字| 文字.is_ascii_digit()))
}
