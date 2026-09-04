//! 受入試験の初期環境を組み立てる: 一時作業域・保管先ルート・origin裸リポジトリ・
//! PC A/PC Bの模擬環境を用意する。番号付きの検査そのものは含まない。項目7・8が使う
//! 独立した模擬PCの組み立ても、この`模擬pcを組み立てる`を共有する。

use crate::acceptance_check::agent_binary::対象実行ファイルパス;
use crate::acceptance_check::object_storage_root::オブジェクト保管ルート;
use crate::acceptance_check::pc_config_dir::PC設定ディレクトリ;
use crate::acceptance_check::pc_environment::模擬PC;
use crate::acceptance_check::scenario_state::{プロファイル名, 主鎖状態};
use crate::acceptance_check::workspace::一時作業域;

pub fn 組み立てる() -> Result<(一時作業域, 主鎖状態, 対象実行ファイルパス), String> {
    let 作業域 = 一時作業域::作成する()?;
    let 実行ファイル = 対象実行ファイルパス::ビルドして解決する()?;

    let storage_a = オブジェクト保管ルート::生成する(作業域.子パス("storage_a"));
    let origin = 作業域.子パス("origin.git");

    let pc_a = 模擬pcを組み立てる(&作業域, "pc-a", &storage_a, &実行ファイル)?;
    let pc_b = 模擬pcを組み立てる(&作業域, "pc-b", &storage_a, &実行ファイル)?;

    let pc_a_workdir = 作業域.子パス("pc_a_workdir");
    let pc_b_workdir = 作業域.子パス("pc_b_workdir");

    let 状態 = 主鎖状態::生成する(storage_a, origin, pc_a, pc_b, pc_a_workdir, pc_b_workdir);
    Ok((作業域, 状態, 実行ファイル))
}

/// 孤立globalconfig・PC設定ディレクトリを持つ模擬PCを1台組み立て、machine全体で1回だけ
/// 行う`git lfs install`(global、hookなし)まで済ませる。項目7・8の独立した模擬PCもこれで作る。
pub fn 模擬pcを組み立てる(
    作業域: &一時作業域,
    名前: &str,
    保管先: &オブジェクト保管ルート,
    実行ファイル: &対象実行ファイルパス,
) -> Result<模擬PC, String> {
    let 分離global設定ファイル = 作業域.子パス(&format!("{名前}_gitconfig"));
    std::fs::write(&分離global設定ファイル, "").map_err(|失敗| format!("{}を作成できなかった: {失敗}", 分離global設定ファイル.display()))?;

    let pc設定 = PC設定ディレクトリ::生成する(作業域.子パス(&format!("{名前}_pcconfig")));
    // 作らずにパスだけ組み立てる。agentが`temp_directory`を読まなくなったことを、
    // 「このディレクトリが最後まで存在しないこと」で確かめるためである（項目3）。
    let 一時ディレクトリ = 作業域.子パス(&format!("{名前}_temp"));
    pc設定.単一プロファイルで準備する(プロファイル名, 保管先, &一時ディレクトリ)?;

    let pc = 模擬PC::生成する(名前, 分離global設定ファイル, pc設定, 一時ディレクトリ, 実行ファイル.clone());
    pc.git実行(作業域.ルート(), &[], &["lfs", "install", "--skip-repo"])?.成功を要求する(&format!("{名前}のgit lfs install"))?;
    Ok(pc)
}

/// origin裸リポジトリを作る。`init.defaultBranch=main`を明示しないと、bare repoの
/// HEAD symrefが既定ブランチ名(環境依存)のまま残り、"main"へpushしても追随しない
/// （「remote HEAD refers to nonexistent ref」の原因になる）。
pub fn 裸リポジトリを作る(pc: &模擬PC, 作業域ルート: &std::path::Path, 先: &std::path::Path) -> Result<(), String> {
    pc.git実行(作業域ルート, &[], &["-c", "init.defaultBranch=main", "init", "--bare", &先.to_string_lossy()])?
        .成功を要求する("origin裸リポジトリの作成")?;
    Ok(())
}
