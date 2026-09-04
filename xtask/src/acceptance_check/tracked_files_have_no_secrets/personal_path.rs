//! 実行中のPCの実際の利用者名を含む、PC固有の実pathの検出。
//!
//! 受入条件9が禁じているのはPC固有の「実」pathである。文書が書式を示すために使う
//! `C:\Users\<ユーザー名>\...` のようなプレースホルダは実pathではない。実行環境の
//! 環境変数から実際の利用者名を取り、その利用者名を含むpathだけを実pathと判定する。
//!
//! 注意: 探す綴りをこのファイルへ直接書くと、このファイル自身が追跡対象になった時点で
//! 検査が自分の探している綴りを自分の中に見つけて不合格になる。利用者ディレクトリの
//! 綴りは断片へ分けて実行時にのみ結合する。

pub struct 実利用者ディレクトリの検出器 {
    小文字化した表記一覧: Vec<String>,
}

impl 実利用者ディレクトリの検出器 {
    pub fn 実行環境から生成する() -> Result<Self, String> {
        let 利用者名 = 環境変数から実行中の利用者名を取得する()?;
        let 小文字の利用者名 = 利用者名.to_lowercase();
        Ok(Self {
            小文字化した表記一覧: vec![
                [&円記号区切りの利用者ディレクトリを組み立てる(), 小文字の利用者名.as_str()].concat(),
                [&斜線区切りの利用者ディレクトリを組み立てる(), 小文字の利用者名.as_str()].concat(),
            ],
        })
    }

    pub fn 検出した記述を列挙する(&self, 内容: &str) -> Vec<String> {
        let 小文字化した内容 = 内容.to_lowercase();
        self.小文字化した表記一覧
            .iter()
            .filter(|表記| 小文字化した内容.contains(表記.as_str()))
            .map(|表記| format!("実行中のPCの利用者ディレクトリ \"{表記}\" を含む"))
            .collect()
    }
}

fn 環境変数から実行中の利用者名を取得する() -> Result<String, String> {
    for 変数名 in ["USERNAME", "USER", "LOGNAME"] {
        if let Ok(値) = std::env::var(変数名)
            && !値.trim().is_empty()
        {
            return Ok(値.trim().to_owned());
        }
    }
    Err("実行中のPCの利用者名を環境変数から取得できず、PC固有の実pathを判定できなかった".to_owned())
}

fn 円記号区切りの利用者ディレクトリを組み立てる() -> String {
    ["c:\\us", "ers\\"].concat()
}

fn 斜線区切りの利用者ディレクトリを組み立てる() -> String {
    ["c:/us", "ers/"].concat()
}
