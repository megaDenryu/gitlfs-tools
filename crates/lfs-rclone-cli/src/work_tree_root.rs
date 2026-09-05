//! Gitリポジトリの作業ツリーのルートを表す値型。役割を持つパスであり、配置の導出を
//! この型のメソッドへ閉じ、呼び出し側で`join`を連ねさせない（グローバルCLAUDE.md
//! 「役割の型は自分の配置を知る」「プリミティブ執着禁止はパス・テキスト・名前にも
//! 適用する」）。

use std::path::{Path, PathBuf};

const プロジェクト設定ファイル名: &str = ".large-assets.toml";
const GITATTRIBUTESファイル名: &str = ".gitattributes";

#[repr(transparent)]
pub(crate) struct 作業ツリールート(PathBuf);

impl 作業ツリールート {
    pub(crate) fn 生成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// このルートに置く`.large-assets.toml`の配置先。
    pub(crate) fn プロジェクト設定ファイルパス(&self) -> PathBuf {
        self.0.join(プロジェクト設定ファイル名)
    }

    /// このルートに置く`.gitattributes`の配置先。
    pub(crate) fn gitattributesファイルパス(&self) -> PathBuf {
        self.0.join(GITATTRIBUTESファイル名)
    }

    /// プロジェクト設定（`.large-assets.toml`）の探索を始める場所。探索は作業ツリーの
    /// ルートから親へ向かって進むため、起点はこのルート自身である。
    pub(crate) fn プロジェクト設定の探索起点(&self) -> PathBuf {
        self.0.clone()
    }

    /// 生のパスへ戻す境界。`git`の子プロセスをこのルートで動かすときにだけ使う
    /// （`std::process::Command::current_dir`と`Gitコマンドの実行場所`への受け渡し）。
    pub(crate) fn パス(&self) -> &Path {
        &self.0
    }
}
