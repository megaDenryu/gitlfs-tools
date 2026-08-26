//! 結合テストの共有ヘルパー。`tests/`配下の各テストファイルは個別の結合テストバイナリ
//! としてコンパイルされ、このモジュールを毎回別々にコンパイルする。どのバイナリも補助関数の
//! 全部は使わないため`dead_code`警告が出るが、これは共有テストヘルパーの既知の性質であり
//! 本体クレートのlintを緩和するものではない（`lfs-rclone-rclone/tests/common/mod.rs`と
//! 同じ扱い）。各テストファイルは`common::fixtures::関数名`のように完全パスで呼び、
//! `use`による再輸出をしない（バイナリごとに未使用importの警告が出るのを避けるため）。

#![allow(dead_code)]

pub mod fixtures;
pub mod payload;
pub mod process;
pub mod rclone_executable;
