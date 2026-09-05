//! 項目3が併せて確かめる事実: ダウンロードの一時ファイルがリポジトリと同じボリュームに
//! 置かれること。
//!
//! Git LFSは`complete`で受け取ったファイルの所有権を奪い、`.git/lfs/objects/`へ`rename`で
//! 移す。一時ファイルがリポジトリと別のドライブにあると、この`rename`が
//! 「別のディスクドライブへは移動できない」で失敗し、`git lfs pull`全体が落ちる。
//!
//! 試験の中で本物の別ドライブを用意することはできない。そこで、置き場所がリポジトリの
//! Gitディレクトリの下にあること（同一ボリュームであることが位置から決まる）と、PC設定へ
//! わざと残した別の置き場所がagentに一度も作られていないことの2つで代える。前者だけだと
//! 「リポジトリ側にも置くが、実際に使うのは設定側」という取り違えを見逃す。

use std::path::Path;

pub fn 確かめる(作業ツリー: &Path, 設定に残した一時ディレクトリ: &Path) -> Result<String, String> {
    let 置き場 = 作業ツリー.join(".git").join("lfs").join("tmp").join("gitlfs-tools");
    if !置き場.is_dir() {
        return Err(format!("ダウンロードの一時ファイル置き場がリポジトリ内に作られていない: {}", 置き場.display()));
    }
    if 設定に残した一時ディレクトリ.exists() {
        return Err(format!(
            "使われないはずのPC設定temp_directoryがagentに作られている: {}",
            設定に残した一時ディレクトリ.display()
        ));
    }
    Ok(format!("一時ファイル置き場はリポジトリ内の{}であり、PC設定のtemp_directoryは使われていない", 置き場.display()))
}
