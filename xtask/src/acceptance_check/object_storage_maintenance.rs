//! `オブジェクト保管ルート`の続きの`impl`。保管先に対する保守操作
//! （数え上げ・削除・破損・複製）を持つ。配置の導出（`object_storage_root.rs`）とは
//! 別の責務のため分ける。

use std::fs;
use std::path::{Path, PathBuf};

use crate::acceptance_check::object_storage_root::オブジェクト保管ルート;

impl オブジェクト保管ルート {
    /// sha256配下に実際に存在するオブジェクトファイルの絶対パス一覧。
    pub fn オブジェクト一覧を数え上げる(&self) -> Result<Vec<PathBuf>, String> {
        let 起点 = self.オブジェクトツリーの起点();
        if !起点.is_dir() {
            return Ok(Vec::new());
        }
        let mut 一覧 = Vec::new();
        再帰的にファイルを集める(&起点, &mut 一覧)?;
        一覧.sort();
        Ok(一覧)
    }

    /// 指定したoidのオブジェクトファイルを削除する（未存在ダウンロード試験用）。
    pub fn オブジェクトを削除する(&self, oid: &str) -> Result<(), String> {
        let パス = self.オブジェクトパス(oid);
        fs::remove_file(&パス).map_err(|失敗| format!("{}を削除できなかった: {失敗}", パス.display()))
    }

    /// 指定したoidのオブジェクトファイルを、SHA-256が一致しない内容で上書きする
    /// （破損ダウンロード試験用）。
    pub fn オブジェクトを破損させる(&self, oid: &str) -> Result<(), String> {
        let パス = self.オブジェクトパス(oid);
        fs::write(&パス, b"corrupted-by-acceptance-check").map_err(|失敗| format!("{}を破損させられなかった: {失敗}", パス.display()))
    }

    /// 保管先の全内容を別ルートへ複製する（バックエンド差し替え試験用）。
    pub fn 複製する(&self, 複製先: &オブジェクト保管ルート) -> Result<(), String> {
        if !self.パス().is_dir() {
            return Ok(());
        }
        ディレクトリを再帰的に複製する(self.パス(), 複製先.パス())
    }
}

fn 再帰的にファイルを集める(対象: &Path, 収集先: &mut Vec<PathBuf>) -> Result<(), String> {
    for 項目 in fs::read_dir(対象).map_err(|失敗| format!("{}を読み取れなかった: {失敗}", 対象.display()))? {
        let 項目 = 項目.map_err(|失敗| format!("項目の読み取りに失敗した: {失敗}"))?;
        let パス = 項目.path();
        if パス.is_dir() {
            再帰的にファイルを集める(&パス, 収集先)?;
        } else {
            収集先.push(パス);
        }
    }
    Ok(())
}

fn ディレクトリを再帰的に複製する(元: &Path, 先: &Path) -> Result<(), String> {
    fs::create_dir_all(先).map_err(|失敗| format!("{}を作成できなかった: {失敗}", 先.display()))?;
    for 項目 in fs::read_dir(元).map_err(|失敗| format!("{}を読み取れなかった: {失敗}", 元.display()))? {
        let 項目 = 項目.map_err(|失敗| format!("項目の読み取りに失敗した: {失敗}"))?;
        let パス = 項目.path();
        let 行き先 = 先.join(項目.file_name());
        if パス.is_dir() {
            ディレクトリを再帰的に複製する(&パス, &行き先)?;
        } else {
            fs::copy(&パス, &行き先).map_err(|失敗| format!("{}を複製できなかった: {失敗}", パス.display()))?;
        }
    }
    Ok(())
}
