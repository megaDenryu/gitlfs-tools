# gitlfs-tools

このリポジトリ（gitlfs-tools）は、複数のPCと複数のGitリポジトリで共用する、大容量バイナリ資産の保管基盤である。

このリポジトリが作るプログラムは、Git LFS（Git Large File Storage。大容量ファイルの実体の代わりに参照情報をGitへ記録する仕組み）の standalone custom transfer agent（Git LFS が単独で起動する、実体の転送だけを引き受けるプログラム）として動作し、マウント済みのクラウドストレージ、または rclone 経由の外部保管先へオブジェクトを保存・取得する。

## 仕組みの概略

```text
各Gitリポジトリ（.gitattributes と Git LFS pointer をコミットする）
（pointerとは、実体の代わりにGitへ記録される参照情報のことである）
  ↓
gitlfs-tools（custom transfer agent。Git LFS が引数なしで起動する）
  ↓
マウント済みのローカルパス、または rclone
  ↓
Google Drive、または別の保管先
```

利用側のプロジェクトがコミットするのは `.gitattributes` と `.large-assets.toml`（スキーマ版と論理プロファイル名だけ）の2ファイルである。**保管先の絶対パス、rcloneのリモート名、認証トークンをプロジェクトは知らない。** 保管先の実体はPCごとの設定だけが持ち、プロジェクトは論理プロファイル名でその設定を参照する。この分離により、2台のPCは保管先の指定を違えたまま同じリポジトリを共有できる。

保管先への書き方は2つある。既定は、Google Drive for Desktop などがドライブとしてマウントした保管先を直接使う方式である。マウントできない保管先には rclone 方式も使える。2つの方式の選び方と設定の書き方は [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) にある。

## 目的別の入口

| やりたいこと | 読む文書 |
|---|---|
| はじめてこのPCへ導入する | [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) |
| 新しいGitリポジトリで使う | [_doc/利用/プロジェクト導入.md](_doc/利用/プロジェクト導入.md) |
| 普段使う。cloneやpushで何が起きるかを知る | [_doc/利用/日常操作.md](_doc/利用/日常操作.md) |
| 失敗した。エラーコードを引く | [_doc/利用/トラブルシューティング.md](_doc/利用/トラブルシューティング.md) |
| このリポジトリのコードを変更する | [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) と [_doc/設計/コード分割規約.md](_doc/設計/コード分割規約.md) |

利用者は、導入を「PC初期設定 → プロジェクト導入」の順に行う。PC初期設定はPCごとに1回、プロジェクト導入はGitリポジトリごとに1回である。

## 状態

v1は、受入条件10（2台のPCで同一コミットから同一の資産を復元する実証）を除いて完成している。仕様の正本は [Issue #2](https://github.com/megaDenryu/gitlfs-tools/issues/2) である。実装のIssue #3 から #9 は完了しており、残る実証は [Issue #10](https://github.com/megaDenryu/gitlfs-tools/issues/10) が扱う。

## 開発者向けの入口

`cargo xtask` が唯一のツール入口である。利用者が引数なしで実行すると、コマンドの一覧が出る。

```powershell
# 実行場所: リポジトリルート
cargo xtask
cargo xtask verify                 # 行数検査 -> エラーコード表の検査 -> cargo check -> clippy -D warnings -> cargo test
cargo xtask check-line-count       # 100行原則と例外台帳で.rsファイルを検査する
cargo xtask check-error-code-table # トラブルシューティング.mdのエラーコード対応表と実装の一致を検査する
cargo xtask check-v1-acceptance    # v1受入条件1〜9番を実git/git-lfs/rcloneで実証する
cargo xtask install-binary         # releaseビルドし、Cargoのbinディレクトリへ実行ファイルを配置する
```

`cargo xtask install-binary` の詳しい動作と、実行ファイルを `target/` の外へ置く理由は [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) の手順5にある。

## 文書一覧（生存型）

常に実装と一致させる義務がある文書である。ここに無い文書は正典ではない。

| 文書 | 内容 |
|---|---|
| [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) | PCごとに1回行う導入。保管先の選択と変更、rcloneを使う場合 |
| [_doc/利用/プロジェクト導入.md](_doc/利用/プロジェクト導入.md) | Gitリポジトリごとに1回行う導入。コミットするファイル、初回clone |
| [_doc/利用/日常操作.md](_doc/利用/日常操作.md) | clone・checkout・pull・pushで何が起きるか、複数PCでの共有、保管先へ到達できないとき |
| [_doc/利用/トラブルシューティング.md](_doc/利用/トラブルシューティング.md) | `doctor`の読み方、エラーコード対応表、`doctor`では検出できない失敗 |
| [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) | 層の定義、依存の向き、主要な設計判断、採用ライブラリ |
| [_doc/設計/コード分割規約.md](_doc/設計/コード分割規約.md) | 役割語彙、昇格経路、規範実装の指名 |
| [_doc/設計/行数の例外台帳.md](_doc/設計/行数の例外台帳.md) | コード行100行の超過を許したファイルの登録簿 |
| [CLAUDE.md](CLAUDE.md) | ツールの入口、文書の置き場所、命名、標準出力の規律 |

`_doc/開発スレッド/` はログ型（追記のみ）であり、索引への登録を要しない。

エラーコード対応表の正本は [_doc/利用/トラブルシューティング.md](_doc/利用/トラブルシューティング.md) にある。実装の正本は `gitlfs-tools-protocol` の `error_code.rs` である。採用した外部ライブラリの一覧と採用理由は [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) の「5. 採用ライブラリ」にある。
