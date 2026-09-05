//! `clone`が作業ツリーを作る先を表す値型。`clone`はこの先で`git lfs install --local`と
//! `install`と`git lfs pull`を続けて動かすため、複製先を`git`任せにせず必ず明示して渡す
//! （場所を確実に知る必要があるため。Issue #11 判断4）。

use std::path::{Path, PathBuf};

use crate::clone::source_url::複製元リポジトリURL;
use crate::work_tree_root::作業ツリールート;

#[repr(transparent)]
pub(crate) struct 複製先ディレクトリ(PathBuf);

impl 複製先ディレクトリ {
    /// 利用者が第2引数で与えた名前から作る。
    pub(crate) fn 指定名から生成する(名前: &str) -> Self {
        Self(PathBuf::from(名前))
    }

    /// 複製元の綴りの末尾から導く。導けない綴りでは`None`を返し、呼び出し側が
    /// 第2引数での指定を促す。
    pub(crate) fn 複製元から導く(複製元: &複製元リポジトリURL) -> Option<Self> {
        複製元.末尾から複製先ディレクトリ名を導く().map(|名前| Self(PathBuf::from(名前)))
    }

    /// `git`の子プロセスへ渡すパス（境界1箇所）。
    pub(crate) fn パス(&self) -> &Path {
        &self.0
    }

    /// 利用者へ示すための表記。
    pub(crate) fn 表示用の綴り(&self) -> String {
        self.0.display().to_string()
    }

    /// `git clone`が成功した後の姿へ変換する。複製先は「これから作る場所」であり、
    /// 複製が成功して初めて作業ツリーのルートになる。この時間の違いを型で分けるため、
    /// 変換を経ずに複製先をそのまま作業ツリーとして扱わせない。
    pub(crate) fn 作業ツリールートへ変換する(&self) -> 作業ツリールート {
        作業ツリールート::生成する(self.0.clone())
    }
}
