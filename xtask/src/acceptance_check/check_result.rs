//! 受入試験1項目分の結果を表すドメインモデル。「期待した結果」と「実際の結果」を
//! 対で持ち、`report.rs`が両方を並べて出す（親からの指示「項目ごとに、期待した結果と
//! 実際の結果を出す」）。

pub struct 検査結果 {
    pub 番号: u8,
    pub 表題: &'static str,
    pub 期待した結果: String,
    pub 状態: 検査状態,
}

pub enum 検査状態 {
    合格 { 実際の結果: String },
    不合格 { 実際の結果: String },
    未実施 { 理由: String },
}

impl 検査結果 {
    /// 検査関数の戻り値（成功時は実際の結果の説明、失敗時は理由）から結果を組み立てる。
    pub fn 生成する(番号: u8, 表題: &'static str, 期待した結果: impl Into<String>, 結果: Result<String, String>) -> Self {
        let 状態 = match 結果 {
            Ok(実際の結果) => 検査状態::合格 { 実際の結果 },
            Err(実際の結果) => 検査状態::不合格 { 実際の結果 },
        };
        Self { 番号, 表題, 期待した結果: 期待した結果.into(), 状態 }
    }

    /// 前提となる項目が失敗したため、この項目自体を実行しなかったことを表す。
    pub fn 未実施(番号: u8, 表題: &'static str, 期待した結果: impl Into<String>, 理由: impl Into<String>) -> Self {
        Self { 番号, 表題, 期待した結果: 期待した結果.into(), 状態: 検査状態::未実施 { 理由: 理由.into() } }
    }

    pub fn 合格したか(&self) -> bool {
        matches!(self.状態, 検査状態::合格 { .. })
    }
}
