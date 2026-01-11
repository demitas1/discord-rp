//! 設定ファイルの管理

use crate::{Activity, Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Discord Application ID
    pub application_id: String,

    /// デフォルトのアクティビティ設定
    #[serde(default)]
    pub activity: Activity,

    /// 起動時に自動接続するか
    #[serde(default = "default_true")]
    pub auto_connect: bool,

    /// 切断時に自動再接続するか
    #[serde(default = "default_true")]
    pub auto_reconnect: bool,

    /// 再接続の間隔（秒）
    #[serde(default = "default_reconnect_interval")]
    pub reconnect_interval: u64,
}

fn default_true() -> bool {
    true
}

fn default_reconnect_interval() -> u64 {
    30
}

impl Default for Config {
    fn default() -> Self {
        Self {
            application_id: String::new(),
            activity: Activity::default(),
            auto_connect: true,
            auto_reconnect: true,
            reconnect_interval: 30,
        }
    }
}

impl Config {
    /// 新しい設定を作成
    pub fn new(application_id: impl Into<String>) -> Self {
        Self {
            application_id: application_id.into(),
            ..Default::default()
        }
    }

    /// ファイルから設定を読み込む
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::ConfigLoadFailed(format!("{}: {}", path.display(), e)))?;

        toml::from_str(&content)
            .map_err(|e| Error::ConfigLoadFailed(format!("TOML パースエラー: {}", e)))
    }

    /// 設定をファイルに保存
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        // 親ディレクトリを作成
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::ConfigSaveFailed(format!("ディレクトリ作成失敗: {}", e)))?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::ConfigSaveFailed(format!("TOML シリアライズエラー: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| Error::ConfigSaveFailed(format!("{}: {}", path.display(), e)))?;

        Ok(())
    }

    /// デフォルトの設定ファイルパスを取得
    pub fn default_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|p| p.join("discord-rp").join("config.toml"))
    }
}
