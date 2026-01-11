//! 設定ファイルの管理

use crate::{Activity, Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Discord Application IDs（キー: インデックス番号）
    #[serde(default)]
    pub application_ids: BTreeMap<u32, String>,

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
            application_ids: BTreeMap::new(),
            activity: Activity::default(),
            auto_connect: true,
            auto_reconnect: true,
            reconnect_interval: 30,
        }
    }
}

impl Config {
    /// 新しい設定を作成
    pub fn new() -> Self {
        Self::default()
    }

    /// Application IDを追加
    pub fn add_application_id(&mut self, index: u32, app_id: impl Into<String>) {
        self.application_ids.insert(index, app_id.into());
    }

    /// 指定インデックスのApplication IDを取得（1始まり）
    pub fn get_application_id(&self, index: u32) -> Result<&str> {
        if index == 0 {
            return Err(Error::InvalidApplicationId(
                "インデックスは1から始まります".to_string(),
            ));
        }

        self.application_ids
            .get(&index)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                Error::InvalidApplicationId(format!(
                    "インデックス {} のApplication IDが登録されていません（登録済み: {:?}）",
                    index,
                    self.application_ids.keys().collect::<Vec<_>>()
                ))
            })
    }

    /// 登録済みのApplication ID数を取得
    pub fn application_id_count(&self) -> usize {
        self.application_ids.len()
    }

    /// 登録済みのインデックス一覧を取得
    pub fn registered_indices(&self) -> Vec<u32> {
        self.application_ids.keys().copied().collect()
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

    /// 環境変数からApplication IDを読み込む
    /// DISCORD_APPLICATION_ID_1, DISCORD_APPLICATION_ID_2, ... の形式
    pub fn load_from_env(&mut self) {
        for i in 1..=100 {
            let key = format!("DISCORD_APPLICATION_ID_{}", i);
            if let Ok(value) = std::env::var(&key) {
                if !value.is_empty() {
                    self.add_application_id(i, value);
                }
            }
        }
    }

    /// 環境変数から設定を作成
    pub fn from_env() -> Self {
        let mut config = Self::new();
        config.load_from_env();
        config
    }
}
