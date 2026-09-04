//! 配置先ディレクトリ（Cargoが実行ファイルを置く`bin`ディレクトリ）の解決と、そこへの配置。
//!
//! 置き場所をCargoの`bin`ディレクトリに定める理由は3つある。`target/`の外にあるので
//! `cargo clean`でも`--release`への切り替えでも消えないこと、rustupの導入時点でPATHへ
//! 通っていること、`CARGO_HOME`という環境変数1つで綴りが決まりOSを問わないことである。
//! xtaskは外部依存を持たないため、`directories`crateは使わず環境変数だけで解決する。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub enum 配置結果 {
    新規配置,
    上書き配置,
    既存を退避して配置 { 退避先: PathBuf },
}

/// 実行ファイルを置くディレクトリ。パスの綴りをこの型のメソッドへ閉じる。
pub struct 実行ファイル配置ディレクトリ(PathBuf);

impl 実行ファイル配置ディレクトリ {
    pub fn 環境変数から解決する() -> Result<Self, String> {
        if let Some(cargoホーム) = 空でない環境変数を読む("CARGO_HOME") {
            return Ok(Self(PathBuf::from(cargoホーム).join("bin")));
        }
        let ホーム = 空でない環境変数を読む("HOME")
            .or_else(|| 空でない環境変数を読む("USERPROFILE"))
            .ok_or_else(|| "配置先を決められない。CARGO_HOME・HOME・USERPROFILEのいずれも設定されていない".to_owned())?;
        Ok(Self(PathBuf::from(ホーム).join(".cargo").join("bin")))
    }

    pub fn ディレクトリのパス(&self) -> &Path {
        &self.0
    }

    pub fn 配置先のパス(&self, 実行ファイル名: &str) -> PathBuf {
        self.0.join(実行ファイル名)
    }

    /// 既存の実行ファイルがあれば上書きする。版を比べないのは、比べるための版の綴りを
    /// 実行ファイルが持っておらず、比較の結果が「最新をビルドして置く」という本コマンドの
    /// 目的を変えないためである。
    pub fn 配置する(&self, 配置元: &Path, 実行ファイル名: &str) -> Result<配置結果, String> {
        fs::create_dir_all(&self.0).map_err(|失敗| format!("配置先ディレクトリを作れなかった: {失敗}"))?;
        let 配置先 = self.配置先のパス(実行ファイル名);
        let 既存あり = 配置先.is_file();

        match fs::copy(配置元, &配置先) {
            Ok(_) if 既存あり => Ok(配置結果::上書き配置),
            Ok(_) => Ok(配置結果::新規配置),
            Err(失敗) if 既存あり => 既存を退避してから配置する(配置元, &配置先, &失敗),
            Err(失敗) => Err(format!("実行ファイルを配置できなかった: {失敗}")),
        }
    }

    /// PATHの各要素と配置先ディレクトリを突き合わせる。書き換えは行わず、案内の材料だけを返す。
    pub fn パスが通っているか(&self) -> bool {
        let Some(パス環境変数) = std::env::var_os("PATH") else {
            return false;
        };
        std::env::split_paths(&パス環境変数).any(|要素| 要素 == self.0)
    }
}

/// Windowsは動作中の実行ファイルを上書きできないが、改名は許す。改名で場所を空けてから
/// 配置し、退避したファイルの削除まで済めば動作中でなかったものとして扱う。
fn 既存を退避してから配置する(配置元: &Path, 配置先: &Path, 直前の失敗: &io::Error) -> Result<配置結果, String> {
    let 退避先 = 配置先.with_extension("previous");
    let _ = fs::remove_file(&退避先);
    fs::rename(配置先, &退避先).map_err(|失敗| {
        format!("既存の実行ファイルを置き換えられなかった。上書きの失敗: {直前の失敗}。退避の失敗: {失敗}")
    })?;
    fs::copy(配置元, 配置先).map_err(|失敗| format!("退避後の配置に失敗した: {失敗}"))?;

    if fs::remove_file(&退避先).is_ok() {
        return Ok(配置結果::上書き配置);
    }
    Ok(配置結果::既存を退避して配置 { 退避先 })
}

fn 空でない環境変数を読む(名前: &str) -> Option<String> {
    std::env::var(名前).ok().filter(|値| !値.trim().is_empty())
}
