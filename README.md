# git-lfs-rclone-storage

複数のPCと複数のGitリポジトリで共用する、大容量バイナリ資産の保管基盤。

Git LFS（Git Large File Storage。大容量ファイルの実体の代わりに参照情報をGitへ記録する仕組み）の
standalone custom transfer agentとして動作し、rclone経由で外部保管先へオブジェクトを保存・取得する。
初期の保管先はGoogle Driveである。

```text
各Gitリポジトリ（.gitattributes と Git LFS pointer）
  ↓
git-lfs-rclone-storage（custom transfer agent）
  ↓
rclone
  ↓
Google Drive、または別のrclone対応先
```

利用側のプロジェクトがコミットするのは`.gitattributes`と`.large-assets.toml`（スキーマ版と
論理プロファイル名だけ）である。Google Drive、認証トークン、PC固有のパスをプロジェクトは知らない。

## 状態

v1を実装中である。仕様の正本は
[Issue #2](https://github.com/megaDenryu/git-lfs-rclone-storage/issues/2)、
実装のIssueは #3 から #8 である。

## 使い方

`cargo xtask` が唯一のツール入口である。引数なしで実行するとコマンドの一覧が出る。

```powershell
# 実行場所: リポジトリルート
cargo xtask verify
```

利用者向けの導入手順は Issue #8 の完了時に本節へ追記する。

## 文書一覧（生存型）

常に実装と一致させる義務がある文書。ここに無い設計文書は正典ではない。

| 文書 | 内容 |
|---|---|
| [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) | 層の定義、依存の向き、主要な設計判断 |
| [_doc/設計/コード分割規約.md](_doc/設計/コード分割規約.md) | 役割語彙、昇格経路、規範実装の指名 |
| [_doc/設計/行数の例外台帳.md](_doc/設計/行数の例外台帳.md) | コード行100行の超過を許したファイルの登録簿 |
| [CLAUDE.md](CLAUDE.md) | ツールの入口、文書の置き場所、命名、標準出力の規律 |

`_doc/開発スレッド/` はログ型（追記のみ）であり、索引への登録を要しない。

## 採用ライブラリ

外部ライブラリは抑制的に採用する。採用したものと理由を次に記す。

| ライブラリ | 用途 | 採用理由 |
|---|---|---|
| serde / serde_json | Git LFS protocolのJSON入出力 | 公開仕様の実装であり、自作しても差別化にならない |
| toml | 設定ファイルの解析 | 同上 |
| sha2 | オブジェクト識別子の検証 | 暗号ハッシュの自作は健全性の検証コストが見合わない |
| uuid | 一時ファイル名の生成 | 衝突しない名前の生成規則は公開仕様である |
| thiserror | エラー型の定義 | 導出マクロのみ。実行時の振る舞いを持たない |
| directories | OSごとの設定ディレクトリの解決 | OSの慣習は第一原理から導出できない蓄積知識である |
| tempfile | テストの隔離ディレクトリ | テスト専用。本体の振る舞いに関わらない |
