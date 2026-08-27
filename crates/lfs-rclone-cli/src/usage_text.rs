//! `help`表示、および起動引数エラー時の案内に使う固定の使い方テキスト。

pub(crate) const 使い方テキスト: &str = "\
git-lfs-rclone-storage - Git LFS custom transfer agent

引数なしで起動すると、Git LFSとのプロトコル通信を1行1JSONで行う。

使い方:
  git-lfs-rclone-storage install [--path <実行ファイルのパス>]
      対象のGitリポジトリへ、このプログラムをcustom transfer agentとして登録する。
      --path省略時は、現在実行中のこの実行ファイル自身の絶対パスを登録する。

  git-lfs-rclone-storage init-project --profile <論理プロファイル名>
      対象リポジトリのルートへ.large-assets.tomlの雛形を作る。
      既存のファイルがある場合は上書きせず失敗する。

  git-lfs-rclone-storage doctor
      現在のリポジトリとPCの設定が揃っているかを確かめ、不足を報告する。

  git-lfs-rclone-storage help
      この使い方を表示する。
";
