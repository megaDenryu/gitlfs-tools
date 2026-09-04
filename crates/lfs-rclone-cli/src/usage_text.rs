//! `help`表示、および起動引数エラー時の案内に使う固定の使い方テキスト。

pub(crate) const 使い方テキスト: &str = "\
git-lfs-rclone-storage - Git LFS custom transfer agent

引数なしで起動すると、Git LFSとのプロトコル通信を1行1JSONで行う。

導入の流れ:
  1. このリポジトリのソースで cargo xtask install-binary を実行する。
     releaseビルドした実行ファイルがCargoのbinディレクトリへ置かれ、そのパスが表示される。
  2. 表示されたパスのこの実行ファイルから、対象のGitリポジトリで install を実行する。
     --path省略時は現在実行中の実行ファイル自身のパスを登録するため、配置先が登録される。
  3. 対象リポジトリで init-project を実行し、最後に doctor で不足を確かめる。

使い方:
  git-lfs-rclone-storage install [--path <実行ファイルのパス>]
      対象のGitリポジトリへ、このプログラムをcustom transfer agentとして登録する。
      --path省略時は、現在実行中のこの実行ファイル自身の絶対パスを登録する。
      target/配下の実行ファイルはcargo cleanで消えるため、配置先から実行すること。

  git-lfs-rclone-storage init-project --profile <論理プロファイル名>
      対象リポジトリのルートへ.large-assets.tomlの雛形を作る。
      既存のファイルがある場合は上書きせず失敗する。

  git-lfs-rclone-storage doctor
      現在のリポジトリとPCの設定が揃っているかを確かめ、不足を報告する。

  git-lfs-rclone-storage help
      この使い方を表示する。
";
