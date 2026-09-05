//! `主鎖状態`の続きの`impl`。9項目とは別に必ず答える所見: 通常の`git clone`だけで
//! LFS対象ファイルを取得できるかを確かめる（Issue #2 7.3節）。PC Bはmachine単位の
//! `git lfs install --skip-repo`は済んでいるが、本エージェントのproject単位の
//! custom transfer登録（`install`サブコマンド）はまだ行っていない状態で試す。

use crate::acceptance_check::lfs_pointer;
use crate::acceptance_check::scenario_state::{主鎖状態, 追跡ファイル名};
use crate::acceptance_check::workspace::一時作業域;

impl 主鎖状態 {
    pub fn 通常cloneの可否を調べる(&self, 作業域: &一時作業域) -> Result<String, String> {
        let 先 = 作業域.子パス("plain_clone_check");
        // 本エージェントのproject単位install(`lfs.standalonetransferagent`のlocal設定)を
        // まだ行っていない、真に「何も設定していない新しいPC」を模すため、この呼び出しだけ
        // config.tomlを持たない空ディレクトリへPC設定の場所を差し替える(誤って模擬PCの
        // 実在する設定を拾って所見が歪まないようにするため)。
        let 未設定pcディレクトリ = 作業域.子ディレクトリ("plain_clone_check_unconfigured_pc")?;
        let 結果 = self.pc_b.git実行(
            作業域.ルート(),
            &[("GITLFS_TOOLS_PC_CONFIG_DIR", 未設定pcディレクトリ.as_os_str())],
            &["clone", &self.origin.to_string_lossy(), &先.to_string_lossy()],
        )?;
        let 終了状態文字列 = if 結果.成功したか { "成功" } else { "失敗" };

        let 対象ファイル = 先.join(追跡ファイル名);
        if !対象ファイル.is_file() {
            return Ok(format!(
                "通常のgit clone(終了状態: {終了状態文字列})では対象ファイル自体が作られなかった。標準エラー出力: {}",
                結果.標準エラー出力.trim()
            ));
        }

        let 内容 = std::fs::read(&対象ファイル).map_err(|失敗| format!("{}を読み取れなかった: {失敗}", 対象ファイル.display()))?;
        let 先頭 = String::from_utf8_lossy(&内容[..内容.len().min(64)]);
        if lfs_pointer::pointer形式か(&先頭) {
            Ok(format!("通常のgit clone(終了状態: {終了状態文字列})はpointer本文({}バイト)を残しただけで、実体は取得できなかった", 内容.len()))
        } else {
            Ok(format!("通常のgit clone(終了状態: {終了状態文字列})だけで実体({}バイト)を取得できた", 内容.len()))
        }
    }
}
