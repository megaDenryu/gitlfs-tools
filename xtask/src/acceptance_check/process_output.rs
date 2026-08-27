//! 子プロセス（git、対象実行ファイル）の実行結果を表す値型。標準出力・標準エラー出力を
//! 文字列として保持し、失敗時にまとめて説明へ埋め込めるようにする。

pub struct 子プロセス出力 {
    pub 成功したか: bool,
    pub 標準出力: String,
    pub 標準エラー出力: String,
}

impl 子プロセス出力 {
    /// 成功していなければ、呼び出し元が渡した説明と両方の出力を含むエラーへ変換する。
    pub fn 成功を要求する(self, 説明: &str) -> Result<Self, String> {
        if self.成功したか {
            Ok(self)
        } else {
            Err(format!(
                "{説明}に失敗した。標準出力: {} / 標準エラー出力: {}",
                self.標準出力.trim(),
                self.標準エラー出力.trim()
            ))
        }
    }
}
