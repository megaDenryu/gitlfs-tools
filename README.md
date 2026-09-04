# git-lfs-rclone-storage

複数のPCと複数のGitリポジトリで共用する、大容量バイナリ資産の保管基盤である。

Git LFS（Git Large File Storage。大容量ファイルの実体の代わりに参照情報をGitへ記録する仕組み）の standalone custom transfer agent として動作し、マウント済みのクラウドストレージ、または rclone 経由の外部保管先へオブジェクトを保存・取得する。

## 仕組みの概略

```text
各Gitリポジトリ（.gitattributes と Git LFS pointer をコミットする）
  ↓
git-lfs-rclone-storage（custom transfer agent。Git LFS が引数なしで起動する）
  ↓
マウント済みのローカルパス、または rclone
  ↓
Google Drive、または別の保管先
```

利用側のプロジェクトがコミットするのは `.gitattributes` と `.large-assets.toml`（スキーマ版と論理プロファイル名だけ）の2ファイルである。**保管先の絶対パス、rcloneのリモート名、認証トークンをプロジェクトは知らない。** 保管先の実体はPCごとの設定だけが持ち、プロジェクトは論理プロファイル名でその設定を参照する。この分離により、2台のPCで保管先の指定が違っていても同じリポジトリを共有できる。

## 目的別の入口

| やりたいこと | 読む文書 |
|---|---|
| はじめてこのPCへ導入する | [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) |
| 新しいGitリポジトリで使う | [_doc/利用/プロジェクト導入.md](_doc/利用/プロジェクト導入.md) |
| 普段使う。cloneやpushで何が起きるかを知る | [_doc/利用/日常操作.md](_doc/利用/日常操作.md) |
| 失敗した。エラーコードを引く | [_doc/利用/トラブルシューティング.md](_doc/利用/トラブルシューティング.md) |
| このリポジトリのコードを変更する | [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) と [_doc/設計/コード分割規約.md](_doc/設計/コード分割規約.md) |

導入は「PC初期設定 → プロジェクト導入」の順に行う。PC初期設定はPCごとに1回、プロジェクト導入はGitリポジトリごとに1回である。

## 状態

v1を実装中である。仕様の正本は [Issue #2](https://github.com/megaDenryu/git-lfs-rclone-storage/issues/2)、実装のIssueは #3 から #8 である。

## 開発者向けのツールの入口

`cargo xtask` が唯一のツール入口である。引数なしで実行するとコマンドの一覧が出る。

```powershell
# 実行場所: リポジトリルート
cargo xtask
cargo xtask verify              # cargo check -> clippy -D warnings -> cargo test
cargo xtask check-line-count    # 100行原則と例外台帳で.rsファイルを検査する
cargo xtask check-v1-acceptance # v1受入条件1〜9番を実git/git-lfs/rcloneで実証する
cargo xtask install-binary      # releaseビルドし、Cargoのbinディレクトリへ実行ファイルを配置する
```

`cargo xtask install-binary` の詳しい動作と、実行ファイルを `target/` の外へ置く理由は [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) の手順5にある。

## 保管先の種類

PC設定（`config.toml`）のプロファイルごとに、保管先へどう書くかを `storage` キーで選ぶ。`storage` を省略したプロファイルは、rcloneを子プロセスとして起動する方式として扱う。

| `storage` | 何をするか | 必要なキー | 書いてはならないキー |
|---|---|---|---|
| `local` | 標準ライブラリのファイル操作だけで、マウント済みのローカルパスへ転送する | `base_path` | `rclone_remote`, `rclone_executable`, `transfer_timeout_seconds` |
| `rclone`（省略時の既定） | rcloneを子プロセスとして起動し、リモートへ転送する | `rclone_remote`, `base_path`, `transfer_timeout_seconds`（`rclone_executable`は任意） | なし |

`local` は、Google Drive for Desktop のようにクラウドストレージがドライブとしてマウントされている場合に選ぶ。転送1件ごとのrcloneの起動（存在確認・転送・最終化で最大4回）が無くなる。`base_path` にはマウント済みの絶対パスを書き、保管先のディレクトリ構成は `rclone` 方式と同じである（`<base_path>/lfs/objects/sha256/<先頭2文字>/<次の2文字>/<oid>`）。

`base_path` が指すディレクトリが存在しないときは、`init` と `doctor` が明示的に失敗する。ドライブがマウントされていない状態で転送を始め、実体がローカルディスクへ溜まるのを防ぐためである。

2つの方式の使い分けは [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) の「2つの方式の違い」にある。

## ダウンロードの一時ファイルの置き場所

agentはダウンロードした実体をいったん一時ファイルへ書き、そのパスを Git LFS へ渡す。Git LFS はそのファイルの所有権を受け取り、リポジトリの `.git/lfs/objects/` へ `rename` で移す。`rename` はボリュームをまたげないため、一時ファイルは必ずリポジトリと同じボリュームに無ければならない。

そのため置き場所はPC設定ではなくリポジトリから決める。`git` へ問い合わせた Git LFS の保管ディレクトリ（既定は `<Gitディレクトリ>/lfs`、Git設定 `lfs.storage` があればそれ）の下の `tmp/rclone-storage-agent` である。利用者が指定する項目は無い。

PC設定の `temp_directory` はこの決定により読まれなくなった。既存の設定ファイルを壊さないため記述は受理し続けるが、値は使わない。`doctor` が残っていることを注記するので、見つけたら削除してよい。

## 文書一覧（生存型）

常に実装と一致させる義務がある文書である。ここに無い文書は正典ではない。

| 文書 | 内容 |
|---|---|
| [_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) | PCごとに1回行う導入。保管先の選択と変更、rcloneを使う場合 |
| [_doc/利用/プロジェクト導入.md](_doc/利用/プロジェクト導入.md) | Gitリポジトリごとに1回行う導入。コミットするファイル、初回clone |
| [_doc/利用/日常操作.md](_doc/利用/日常操作.md) | clone・checkout・pull・pushで何が起きるか、複数PC、offline |
| [_doc/利用/トラブルシューティング.md](_doc/利用/トラブルシューティング.md) | `doctor`の読み方、エラーコード対応表、`doctor`では検出できない失敗 |
| [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) | 層の定義、依存の向き、主要な設計判断 |
| [_doc/設計/コード分割規約.md](_doc/設計/コード分割規約.md) | 役割語彙、昇格経路、規範実装の指名 |
| [_doc/設計/行数の例外台帳.md](_doc/設計/行数の例外台帳.md) | コード行100行の超過を許したファイルの登録簿 |
| [CLAUDE.md](CLAUDE.md) | ツールの入口、文書の置き場所、命名、標準出力の規律 |

`_doc/開発スレッド/` はログ型（追記のみ）であり、索引への登録を要しない。

エラーコード対応表の正本は [_doc/利用/トラブルシューティング.md](_doc/利用/トラブルシューティング.md) にある。実装の正本は `lfs-rclone-protocol` の `error_code.rs` である。

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
