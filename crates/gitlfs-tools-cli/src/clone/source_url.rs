//! `clone`サブコマンドが受け取る複製元の綴りを表す値型。形式を持つテキストであり、
//! 末尾からディレクトリ名を導く規則をこの型のメソッドへ閉じる（グローバルCLAUDE.md
//! 「プリミティブ執着禁止はパス・テキスト・名前にも適用する」）。
//!
//! 綴りの妥当性はこの型では判定しない。到達できるかどうかを決めるのは`git`であり、
//! agentが独自の判定を持つと`git`が受理する綴りを取りこぼすためである。

/// 複製元の綴りの中で、ホスト・経路・名前を分ける文字。
const 区切り文字: [char; 3] = ['/', '\\', ':'];

#[repr(transparent)]
pub(crate) struct 複製元リポジトリURL(String);

impl 複製元リポジトリURL {
    pub(crate) fn 生成する(綴り: impl Into<String>) -> Self {
        Self(綴り.into())
    }

    /// `git clone`へ渡す文字列表現（境界1箇所）。
    pub(crate) fn 文字列表現(&self) -> &str {
        &self.0
    }

    /// 末尾から複製先のディレクトリ名を導く。末尾の区切り文字を落とし、最後の区切りより
    /// 後ろを取り、末尾の`.git`を外す。名前として使えない綴りでは`None`を返し、呼び出し側が
    /// 第2引数での指定を促す。
    ///
    /// 区切りを1つも含まない綴り（`https://github.com/`のようにホストだけの綴り）からは
    /// 導かない。ホスト名を複製先のディレクトリ名にすると、利用者が意図しない名前の
    /// ディレクトリが黙って作られるためである。
    pub(crate) fn 末尾から複製先ディレクトリ名を導く(&self) -> Option<String> {
        let 末尾の区切りを落とした綴り = self.0.trim_end_matches(['/', '\\']);
        let 通信手段を外した綴り = match 末尾の区切りを落とした綴り.split_once("://") {
            Some((_, 後ろ)) => 後ろ,
            None => 末尾の区切りを落とした綴り,
        };
        if !通信手段を外した綴り.contains(区切り文字) {
            return None;
        }
        let 最後の区切りより後ろ = 通信手段を外した綴り.rsplit(区切り文字).next()?;
        let 名前 = 最後の区切りより後ろ.strip_suffix(".git").unwrap_or(最後の区切りより後ろ);
        if 名前.is_empty() || 名前 == "." || 名前 == ".." { None } else { Some(名前.to_owned()) }
    }
}

#[cfg(test)]
mod テスト {
    use super::複製元リポジトリURL;

    fn 導いた名前(綴り: &str) -> Option<String> {
        複製元リポジトリURL::生成する(綴り).末尾から複製先ディレクトリ名を導く()
    }

    #[test]
    fn 末尾のgit拡張子と区切り文字を外してディレクトリ名を導く() {
        assert_eq!(導いた名前("https://github.com/owner/repo.git").as_deref(), Some("repo"));
        assert_eq!(導いた名前("https://github.com/owner/repo").as_deref(), Some("repo"));
        assert_eq!(導いた名前("https://github.com/owner/repo/").as_deref(), Some("repo"));
        assert_eq!(導いた名前("git@github.com:owner/repo.git").as_deref(), Some("repo"));
        assert_eq!(導いた名前("C:/devs/origin.git").as_deref(), Some("origin"));
    }

    #[test]
    fn 名前として使えない綴りでは導出できない() {
        assert_eq!(導いた名前(""), None);
        assert_eq!(導いた名前("https://github.com/"), None, "ホストだけの綴りからは導かない");
        assert_eq!(導いた名前("../"), None);
    }
}
