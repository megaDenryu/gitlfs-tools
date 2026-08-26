# CLAUDE.md — git-lfs-rclone-storage

グローバルCLAUDE.md（`~/.claude/CLAUDE.md`）を前提とし、本リポジトリ固有の事項だけを書く。
Rustの書き方は `~/.claude/skills/rust` を、層の決め方は `~/.claude/skills/アーキテクチャ先行` を、
層の中の置き場所は `~/.claude/skills/layer-roles` を参照する。

## このリポジトリは何か

複数のPCと複数のGitリポジトリで共用する、大容量バイナリ資産の保管基盤である。Git LFS
（Git Large File Storage。大容量ファイルの実体の代わりに参照情報をGitへ記録する仕組み）の
standalone custom transfer agentとして動作し、rclone経由で外部保管先へオブジェクトを
保存・取得する。初期の保管先はGoogle Driveである。

仕様の正本はIssue #2である。実装Issueは #3 から #8 で、#2 のsub-issueとして登録されている。

## ツールの入口

**`cargo xtask`** が唯一の入口である。引数なしで実行すると全コマンドの一覧が出る。

```powershell
# 実行場所: リポジトリルート
cargo xtask
cargo xtask verify   # cargo check -> clippy -D warnings -> cargo test
```

新しい手順を作ったら、シェルスクリプトや個別のコマンド列ではなく`xtask`のサブコマンドとして
登録する。登録なきツールの作成は禁止する。

## 文書の置き場所

文書は2種類に分ける。索引は **[README.md](README.md)** である。索引に登録するまでが作成である。

- **生存型**（常に実装と一致させる義務がある）: `_doc/設計/` に置き、README.mdの文書一覧へ登録する。
  実装とずれたら実装に合わせて文書を直す。
- **ログ型**（追記のみ。過去の記述は古くならない）: `_doc/開発スレッド/` に置く。索引への登録は不要。

## 層と依存の向き

[_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) が正本である。コードを書く前に読む。

層はクレートで表し、依存の向きはCargo.tomlが強制する。層の中のファイルの切り方は
[_doc/設計/コード分割規約.md](_doc/設計/コード分割規約.md) に従う。

## 命名

Rust内部で意味を表す識別子は日本語で書く。受理条件はグローバルCLAUDE.md「日本語命名の受理条件」に従う。

外部仕様が名前を決めている次のものは英語のまま保持する。独自に翻訳しない。

- Git LFS custom transfer protocolのJSONフィールド名・イベント名（`event`, `oid`, `size`, `path`, `init`, `upload`, `download`, `terminate`, `complete`）
- rcloneのCLI引数（`lsjson`, `copyto`, `moveto` 等）
- TOMLのkey（`schema_version`, `profile`, `rclone_remote`, `base_path`, `temp_directory`, `transfer_timeout_seconds`）
- Git設定のkey（`lfs.customtransfer.rclone-storage.path` 等）
- Cargoのパッケージ名

## 標準出力の規律

**標準出力はGit LFSとの通信専用である。** 1行1JSONを書き、各行の直後にflushする。

- 診断情報・ログはすべて標準エラー出力へ書く。
- `println!` を `lfs-rclone-cli` と `lfs-rclone-protocol` の外で使わない。
- rcloneの標準出力・標準エラー出力は捕捉し、agentの標準出力へ流さない。

この規律を破るとGit LFS側のJSON解析が壊れ、転送全体が失敗する。

## 認証情報を混入させない

- rclone設定、OAuthトークン、client secret、PC設定、ログ、試験用の資格情報をGit管理対象にしない。
- サンプル設定には架空のリモート名とパスだけを書く。秘密値の欄を設けない。
- エラー表示に不足している論理プロファイル名は出してよいが、設定の全量・トークン・
  PC固有の絶対パスを出さない。
- コミット前に `git diff --cached` を目視し、絶対パスと資格情報が入っていないことを確かめる。

## 日本語ファイル名のモジュール宣言

日本語のファイル名を持つモジュールは、`#[path]` で綴りを明示しないとコンパイルできない
（rustc E0754）。

```rust
#[path = "コマンド定義.rs"]
mod コマンド定義;
```

またモジュール名と、その中の型名を同じ綴りにすると名前が衝突する（rustc E0255）。モジュールは
ファイルのまとまりを表す名前にし、型名と重ねない。規範実装は `xtask/src/main.rs` である。

## 検証

コミット前に `cargo xtask verify` を完走させる。lintの緩和（`unwrap_used`等のdenyを外す変更）は禁止する。
テストモジュールに限り `#[allow(clippy::unwrap_used)]` を局所的に付けてよい。

実rcloneを使う結合テストは、資格情報が不要なlocal backendで行う。実Google Driveを使う試験は
通常の自動テストから分離し、手動で実行する。
