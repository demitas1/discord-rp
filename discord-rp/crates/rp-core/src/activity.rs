//! Discord Rich Presence アクティビティの定義

use serde::{Deserialize, Serialize};

/// Rich Presenceのアクティビティ設定
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Activity {
    /// 詳細テキスト（1行目）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// 状態テキスト（2行目）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// タイムスタンプ設定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,

    /// アセット設定（画像）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
}

/// タイムスタンプ設定
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivityTimestamps {
    /// 開始時刻（Unixタイムスタンプ、秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,

    /// 終了時刻（Unixタイムスタンプ、秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

/// アセット設定（画像）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivityAssets {
    /// 大きい画像のキー（Developer Portalで設定した名前）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,

    /// 大きい画像のツールチップテキスト
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,

    /// 小さい画像のキー
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,

    /// 小さい画像のツールチップテキスト
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

impl Activity {
    /// 新しいアクティビティを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// 詳細テキストを設定
    pub fn details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// 状態テキストを設定
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// 開始時刻を現在時刻に設定
    pub fn start_timestamp(mut self) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        self.timestamps = Some(ActivityTimestamps {
            start: Some(now),
            end: None,
        });
        self
    }

    /// タイムスタンプを設定
    pub fn timestamps(mut self, timestamps: ActivityTimestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    /// アセットを設定
    pub fn assets(mut self, assets: ActivityAssets) -> Self {
        self.assets = Some(assets);
        self
    }

    /// 大きい画像を設定
    pub fn large_image(mut self, key: impl Into<String>, text: Option<String>) -> Self {
        let assets = self.assets.get_or_insert_with(ActivityAssets::default);
        assets.large_image = Some(key.into());
        assets.large_text = text;
        self
    }

    /// 小さい画像を設定
    pub fn small_image(mut self, key: impl Into<String>, text: Option<String>) -> Self {
        let assets = self.assets.get_or_insert_with(ActivityAssets::default);
        assets.small_image = Some(key.into());
        assets.small_text = text;
        self
    }
}

impl ActivityTimestamps {
    /// 現在時刻から開始するタイムスタンプを作成
    pub fn from_now() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        Self {
            start: Some(now),
            end: None,
        }
    }
}
