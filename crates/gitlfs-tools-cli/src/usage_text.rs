//! `help`表示、および起動引数エラー時の案内に使う固定の使い方テキスト。

pub(crate) const 使い方テキスト: &str = "\
gitlfs-tools - Git LFS custom transfer agent

引数なしで起動すると、Git LFSとのプロトコル通信を1行1JSONで行う。

導入の流れ:
  1. このリポジトリのソースで cargo xtask install-binary を実行する。
     releaseビルドした実行ファイルがCargoのbinディレクトリへ置かれ、そのパスが表示される。
  2. 表示されたパスのこの実行ファイルから、対象のGitリポジトリで install を実行する。
     --path省略時は現在実行中の実行ファイル自身のパスを登録するため、配置先が登録される。
  3. 対象リポジトリで init-project を実行し、最後に doctor で不足を確かめる。

使い方:
  gitlfs-tools install [--path <実行ファイルのパス>]
      対象のGitリポジトリへ、このプログラムをcustom transfer agentとして登録する。
      --path省略時は、現在実行中のこの実行ファイル自身の絶対パスを登録する。
      target/配下の実行ファイルはcargo cleanで消えるため、配置先から実行すること。

  gitlfs-tools init-project --profile <論理プロファイル名>
      対象リポジトリのルートへ.large-assets.tomlの雛形を作る。
      既存のファイルがある場合は上書きせず失敗する。

  gitlfs-tools doctor
      現在のリポジトリとPCの設定が揃っているかを確かめ、不足を報告する。

  gitlfs-tools check-objects [--all]
      Git LFSが参照するオブジェクトが保管先に実在するかを突き合わせ、欠けているものを
      一覧で示す。--allを付けると全履歴(過去の版を含む)を対象にする。
      保管先に在るがGit LFSが参照しないオブジェクトは、他のリポジトリが置いたものであり
      正常なため報告しない。

  gitlfs-tools clone <リポジトリのURL> [<ディレクトリ名>]
      初回cloneの4手順(clone・git lfs install --local・install・git lfs pull)を1コマンドで行う。
      ディレクトリ名を省略すると、URLの末尾から導いて表示する。
      git cloneの他の引数(--branch・--depth等)は通さない。それらが要る場合は
      _doc/利用/プロジェクト導入.mdに残した従来の4手順を使う。
      途中で失敗しても、複製した作業ツリーは消さずに残す。

  gitlfs-tools help
      この使い方を表示する。
";
