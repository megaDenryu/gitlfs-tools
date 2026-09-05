//! 発行するタグの名前。`v<版>`という綴りをこの型の中の1箇所へ閉じる。
//!
//! 注意: この綴りは`.github/workflows/release.yml`が起点にするタグの形（`v*`）と
//! 対になっている。片方だけを変えると、タグを送ってもReleaseが作られなくなる。

use crate::release::workspace_version::ワークスペースの版;

/// GitHubのReleaseの起点になるタグの名前。
pub struct リリースタグ名(String);

impl リリースタグ名 {
    pub fn 版から組み立てる(版: &ワークスペースの版) -> Self {
        Self(format!("v{}", 版.版の文字列()))
    }

    pub fn タグ名の文字列(&self) -> &str {
        &self.0
    }
}
