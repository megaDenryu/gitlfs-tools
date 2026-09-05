//! テスト専用の偽rclone実行ファイル。ファイルだけで挙動を制御する。
//!
//! 前提: 本クレートの`tests/`配下からのみ起動される。環境変数は使わない
//! （`std::env::set_var`はこのワークスペースでは`unsafe`扱いであり、`unsafe_code = "forbid"`
//! の下では使えない。加えて環境変数はプロセス全体で共有されるため、並行実行される
//! テストどうしが値を取り合ってしまう）。
//!
//! 受け取った引数の中から`/lfs/`を含むもの（保管先オブジェクトパス。境界のパス規約に
//! 常に含まれる区切りである）を1つ探し、そこから呼び出し元テストが選んだ「保管先基底パス」
//! の文字列を復元する。その文字列をディレクトリ名にして
//! `<一時ディレクトリ>/gitlfs_tools_fake_rclone_test/<基底パス>/`を指示置き場とする。
//! 呼び出し元テストは基底パスをテストごとに一意な文字列にすることで、指示置き場が
//! 他のテストと衝突しないようにする（ロックを持たずに並行実行できる）。

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

const 区切り: &str = "/lfs/";
const 指示置き場の親: &str = "gitlfs_tools_fake_rclone_test";

fn main() {
    let 引数: Vec<String> = env::args().skip(1).collect();
    let 指示置き場 = 指示置き場を求める(&引数);

    引数を記録する(&指示置き場, &引数);
    指定があれば眠る(&指示置き場);

    if let Some(終了コード) = 即終了で応答する(&指示置き場) {
        std::process::exit(終了コード);
    }

    match 引数.first().map(String::as_str).unwrap_or("") {
        "lsjson" => 存在確認へ応答する(&指示置き場),
        "moveto" => 最終化を記録する(&指示置き場),
        _ => {}
    }

    std::process::exit(0);
}

fn 指示置き場を求める(引数: &[String]) -> Option<PathBuf> {
    let 対象引数 = 引数.iter().find(|引数| 引数.contains(区切り))?;
    let (左, _右) = 対象引数.split_once(区切り)?;
    let 基底パス = 左.rsplit_once(':').map_or(左, |(_remote, path)| path);
    Some(env::temp_dir().join(指示置き場の親).join(基底パス))
}

fn 指示ファイルを読む(指示置き場: &Option<PathBuf>, ファイル名: &str) -> Option<String> {
    let ディレクトリ = 指示置き場.as_ref()?;
    fs::read_to_string(ディレクトリ.join(ファイル名)).ok()
}

fn 引数を記録する(指示置き場: &Option<PathBuf>, 引数: &[String]) {
    let Some(ディレクトリ) = 指示置き場 else {
        return;
    };
    if fs::create_dir_all(ディレクトリ).is_err() {
        return;
    }
    let 行 = 引数.join("\u{1f}");
    if let Ok(mut ファイル) = fs::OpenOptions::new().create(true).append(true).open(ディレクトリ.join("args_log")) {
        let _ = writeln!(ファイル, "{行}");
    }
}

fn 指定があれば眠る(指示置き場: &Option<PathBuf>) {
    let Some(ミリ秒文字列) = 指示ファイルを読む(指示置き場, "sleep_ms") else {
        return;
    };
    if let Ok(ミリ秒) = ミリ秒文字列.trim().parse::<u64>() {
        std::thread::sleep(Duration::from_millis(ミリ秒));
    }
}

fn 即終了で応答する(指示置き場: &Option<PathBuf>) -> Option<i32> {
    let 終了コード文字列 = 指示ファイルを読む(指示置き場, "exit_code")?;
    if let Some(標準エラー) = 指示ファイルを読む(指示置き場, "stderr") {
        eprint!("{標準エラー}");
    }
    if let Some(標準出力) = 指示ファイルを読む(指示置き場, "stdout") {
        print!("{標準出力}");
    }
    Some(終了コード文字列.trim().parse().unwrap_or(1))
}

fn 存在確認へ応答する(指示置き場: &Option<PathBuf>) {
    let 見つかった = 指示置き場
        .as_ref()
        .is_some_and(|ディレクトリ| ディレクトリ.join("marker").exists());

    if 見つかった {
        let サイズ = 指示ファイルを読む(指示置き場, "present_size").unwrap_or_else(|| "0".to_owned());
        println!("[{{\"Size\":{サイズ}}}]");
    } else {
        println!("[]");
    }
}

fn 最終化を記録する(指示置き場: &Option<PathBuf>) {
    let Some(ディレクトリ) = 指示置き場 else {
        return;
    };
    let _ = fs::create_dir_all(ディレクトリ);
    let _ = fs::write(ディレクトリ.join("marker"), b"");

    // moveto成功後にlsjsonが返すべきバイト数。テストが`finalize_size`へ事前に書いた値を
    // そのまま`present_size`へ引き継ぐ。偽実行ファイルは実際のバイト列を転送しないため、
    // このファイルなしではmoveto後のサイズを知りようがない。
    let 最終化後のバイト数 = fs::read_to_string(ディレクトリ.join("finalize_size")).unwrap_or_else(|_| "0".to_owned());
    let _ = fs::write(ディレクトリ.join("present_size"), 最終化後のバイト数);
}
