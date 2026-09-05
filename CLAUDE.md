# CLAUDE.md — gitlfs-tools

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

## 行数

コード行で1ファイル100行以内、1関数50行以内が原則である。**空行と、コメントだけの行は数えない。** 行末に書いたコメントはコードの行として数える。

100行に収めるためだけの分割は禁止する。統合した結果100行を超えるなら統合してよいが、150行が上限であり、[_doc/設計/行数の例外台帳.md](_doc/設計/行数の例外台帳.md) への登録が要る。判定の全文はグローバルCLAUDE.md「1ファイル100行の原則と分割の質」を読む。

```powershell
# 実行場所: リポジトリルート
cargo xtask check-line-count
```

## 命名

Rust内部で意味を表す識別子は日本語で書く。受理条件はグローバルCLAUDE.md「日本語命名の受理条件」に従う。

外部仕様が名前を決めている次のものは英語のまま保持する。独自に翻訳しない。

- Git LFS custom transfer protocolのJSONフィールド名・イベント名（`event`, `oid`, `size`, `path`, `init`, `upload`, `download`, `terminate`, `complete`）
- rcloneのCLI引数（`lsjson`, `copyto`, `moveto` 等）
- TOMLのkey（`schema_version`, `profile`, `rclone_remote`, `base_path`, `temp_directory`, `transfer_timeout_seconds`）
- Git設定のkey（`lfs.customtransfer.gitlfs-tools.path` 等）
- Cargoのパッケージ名

## 標準出力の規律

**標準出力はGit LFSとの通信専用である。** 1行1JSONを書き、各行の直後にflushする。

- 診断情報・ログはすべて標準エラー出力へ書く。
- `println!` を `gitlfs-tools-cli` と `gitlfs-tools-protocol` の外で使わない。
- rcloneの標準出力・標準エラー出力は捕捉し、agentの標準出力へ流さない。

この規律を破るとGit LFS側のJSON解析が壊れ、転送全体が失敗する。

## 認証情報を混入させない

- rclone設定、OAuthトークン、client secret、PC設定、ログ、試験用の資格情報をGit管理対象にしない。
- サンプル設定には架空のリモート名とパスだけを書く。秘密値の欄を設けない。
- エラー表示に不足している論理プロファイル名は出してよいが、設定の全量・トークン・
  PC固有の絶対パスを出さない。
- コミット前に `git diff --cached` を目視し、絶対パスと資格情報が入っていないことを確かめる。

## ファイル名と識別子の言語

**ファイル名・ディレクトリ名は英語の `snake_case`**、**中身の識別子は日本語**とする。既存のRustリポジトリ（Blitzdrache0、GameScriptingTheory）と同じ慣習である。

```rust
// crates/gitlfs-tools-domain/src/object_identifier.rs
pub struct オブジェクト識別子(String);
```

モジュール名は英語のファイル名と一致する。英語のモジュール名は読み手にとって補完力がゼロであるため（グローバルCLAUDE.md「日本語命名の受理条件」条2）、その中に置く日本語の型名・関数名は、モジュール名による補完を当てにせず名前単独で「これは何か」に答えられる綴りにする。ファイル名が英語であることを、日本語識別子を短くしてよい理由にしない。

日本語ファイル名を使わないため、`#[path]` によるモジュール宣言（rustc E0754 の回避）は不要である。詳細は `~/.claude/skills/rust` 第8節を参照する。

## 検証

コミット前に `cargo xtask verify` を完走させる。lintの緩和（`unwrap_used`等のdenyを外す変更）は禁止する。
テストモジュールに限り `#[allow(clippy::unwrap_used)]` を局所的に付けてよい。

手動の動作確認に、担当者は実マウント先（PC設定の `personal-large-assets` が指す保管先）を使わない。担当者は試験用のプロファイル `test-large-assets` を使う（[_doc/利用/PC初期設定.md](_doc/利用/PC初期設定.md) の「試験用のプロファイルを別に定義する」）。実マウント先は複数のGitリポジトリが共用するため、後片づけが他のリポジトリの実体を消す。

担当者は、保管先の `lfs` ディレクトリを消すコマンドを書かない。後片づけは、担当者が自分で作ったオブジェクトのパスを名指しで消す形にする。

実マウント先が要るのは、担当者が Google Drive for Desktop 自身の挙動（クラウドへの同期の遅れ、ストリーミング状態（実体がクラウドにあり、読むときに取りに行く状態）のファイルの読み出し、ドライブ文字の割り当て）を調べるときだけである。基盤の実装の検証にマウントは要らない（`gitlfs-tools-local` は標準ライブラリのファイル操作しか使わない）。担当者がその調査を行う場合も、保管先の根ではなく調査専用のフォルダを作り、その中だけで行う。

`cargo xtask check-v1-acceptance` は `std::env::temp_dir()` の下の一時作業域を保管先に見立てて動くため、この規約の対象外である。

実rcloneを使う結合テストは、資格情報が不要なlocal backendで行う。実Google Driveを使う試験は
通常の自動テストから分離し、手動で実行する。
