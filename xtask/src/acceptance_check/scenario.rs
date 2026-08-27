//! 受入試験全体の入口。初期環境を組み立て、項目1〜6の連鎖、項目7・8・9を実行し、
//! 一時作業域を必ず片づけてから結果を報告する。

use crate::acceptance_check::check_result::検査結果;
use crate::acceptance_check::{download_fails_on_missing_or_corrupt, report, restore_after_backend_replacement, scenario_chain_runner, scenario_setup, tracked_files_have_no_secrets};

pub fn 実行する() -> Result<(), String> {
    let (作業域, mut 状態, 実行ファイル) = scenario_setup::組み立てる()?;

    let (mut 結果一覧, 追加所見) = scenario_chain_runner::主鎖を実行する(&mut 状態, &作業域);

    結果一覧.push(検査結果::生成する(
        7,
        "保管先の削除・破損でdownloadが明示的に失敗する",
        "削除・破損させたオブジェクトのdownloadは失敗し、working treeへ別の内容を置かない",
        download_fails_on_missing_or_corrupt::実行する(&作業域, &実行ファイル),
    ));
    結果一覧.push(検査結果::生成する(
        8,
        "保管先backendを差し替えても同じcommitを復元できる",
        "Git側を変更せず、複製した新backendから同じcommitの内容を復元できる",
        restore_after_backend_replacement::実行する(&状態, &作業域, &実行ファイル),
    ));
    結果一覧.push(検査結果::生成する(
        9,
        "tracked filesに認証情報とPC固有の実pathを含まない",
        "tracked filesを検査しても認証情報とPC固有の実pathを含まない",
        tracked_files_have_no_secrets::実行する(),
    ));

    // 前提: `LFS_RCLONE_ACCEPT_KEEP_WORKSPACE`を設定すると一時作業域を削除せずに残す。
    // 失敗を再現・調査する開発者専用の脱出口であり、既定(未設定)では常に片づける。
    let 後始末結果 = if std::env::var("LFS_RCLONE_ACCEPT_KEEP_WORKSPACE").is_ok() { Ok(()) } else { 作業域.後始末する() };
    後始末結果と検査結果をまとめて報告する(&結果一覧, &追加所見, 後始末結果)
}

fn 後始末結果と検査結果をまとめて報告する(結果一覧: &[検査結果], 追加所見: &[String], 後始末結果: Result<(), String>) -> Result<(), String> {
    let 報告結果 = report::結果を報告する(結果一覧, 追加所見);
    match (報告結果, 後始末結果) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(片付け失敗)) => Err(format!("検査は全て合格したが、一時作業域の後始末に失敗した: {片付け失敗}")),
        (Err(検査失敗), Ok(())) => Err(検査失敗),
        (Err(検査失敗), Err(片付け失敗)) => Err(format!("{検査失敗}(さらに一時作業域の後始末にも失敗した: {片付け失敗})")),
    }
}
