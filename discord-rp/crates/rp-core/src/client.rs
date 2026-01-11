//! Discord Rich Presence クライアント

use crate::{Activity, Error, Result};
use discord_rich_presence::{activity as discord_activity, DiscordIpc, DiscordIpcClient};
use std::time::Instant;
use tracing::{debug, error, info, warn};

/// 最小更新間隔（秒）
const MIN_UPDATE_INTERVAL_SECS: u64 = 15;

/// Rich Presenceクライアント
pub struct RichPresenceClient {
    client: DiscordIpcClient,
    application_id: String,
    connected: bool,
    last_update: Option<Instant>,
}

impl RichPresenceClient {
    /// 新しいクライアントを作成
    pub fn new(application_id: impl Into<String>) -> Result<Self> {
        let app_id = application_id.into();

        if app_id.is_empty() {
            return Err(Error::InvalidApplicationId(
                "Application ID が空です".to_string(),
            ));
        }

        let client = DiscordIpcClient::new(&app_id)
            .map_err(|e| Error::InvalidApplicationId(e.to_string()))?;

        Ok(Self {
            client,
            application_id: app_id,
            connected: false,
            last_update: None,
        })
    }

    /// Application IDを取得
    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    /// 接続状態を取得
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Discord IPCに接続
    pub fn connect(&mut self) -> Result<()> {
        if self.connected {
            debug!("既に接続済みです");
            return Ok(());
        }

        info!("Discord に接続中...");

        self.client.connect().map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("No such file or directory")
                || err_str.contains("Connection refused")
            {
                Error::DiscordNotRunning
            } else {
                Error::ConnectionFailed(err_str)
            }
        })?;

        self.connected = true;
        info!("Discord に接続しました");
        Ok(())
    }

    /// Discord IPCから切断
    pub fn disconnect(&mut self) -> Result<()> {
        if !self.connected {
            debug!("既に切断済みです");
            return Ok(());
        }

        info!("Discord から切断中...");

        self.client
            .close()
            .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        self.connected = false;
        self.last_update = None;
        info!("Discord から切断しました");
        Ok(())
    }

    /// アクティビティを更新
    pub fn update_activity(&mut self, activity: &Activity) -> Result<()> {
        if !self.connected {
            return Err(Error::Disconnected);
        }

        // Rate limit チェック
        if let Some(last) = self.last_update {
            let elapsed = last.elapsed().as_secs();
            if elapsed < MIN_UPDATE_INTERVAL_SECS {
                warn!(
                    "更新間隔が短すぎます（{}秒経過、最低{}秒必要）",
                    elapsed, MIN_UPDATE_INTERVAL_SECS
                );
                return Err(Error::RateLimited);
            }
        }

        debug!("アクティビティを更新中: {:?}", activity);

        let discord_activity = Self::build_discord_activity(activity);

        self.client
            .set_activity(discord_activity)
            .map_err(|e| Error::ActivityUpdateFailed(e.to_string()))?;

        self.last_update = Some(Instant::now());
        info!("アクティビティを更新しました");
        Ok(())
    }

    /// アクティビティをクリア
    pub fn clear_activity(&mut self) -> Result<()> {
        if !self.connected {
            return Err(Error::Disconnected);
        }

        debug!("アクティビティをクリア中...");

        self.client
            .clear_activity()
            .map_err(|e| Error::ActivityUpdateFailed(e.to_string()))?;

        self.last_update = Some(Instant::now());
        info!("アクティビティをクリアしました");
        Ok(())
    }

    /// 再接続を試みる
    pub fn reconnect(&mut self) -> Result<()> {
        info!("再接続を試みています...");

        if self.connected {
            if let Err(e) = self.disconnect() {
                warn!("切断中にエラーが発生: {}", e);
            }
        }

        // 新しいクライアントを作成して再接続
        self.client = DiscordIpcClient::new(&self.application_id)
            .map_err(|e| Error::InvalidApplicationId(e.to_string()))?;

        self.connect()
    }

    /// Activityをdiscord-rich-presenceのActivity型に変換
    fn build_discord_activity(activity: &Activity) -> discord_activity::Activity<'_> {
        let mut da = discord_activity::Activity::new();

        if let Some(ref details) = activity.details {
            da = da.details(details);
        }

        if let Some(ref state) = activity.state {
            da = da.state(state);
        }

        if let Some(ref ts) = activity.timestamps {
            let mut timestamps = discord_activity::Timestamps::new();
            if let Some(start) = ts.start {
                timestamps = timestamps.start(start);
            }
            if let Some(end) = ts.end {
                timestamps = timestamps.end(end);
            }
            da = da.timestamps(timestamps);
        }

        if let Some(ref assets) = activity.assets {
            let mut discord_assets = discord_activity::Assets::new();
            if let Some(ref large_image) = assets.large_image {
                discord_assets = discord_assets.large_image(large_image);
            }
            if let Some(ref large_text) = assets.large_text {
                discord_assets = discord_assets.large_text(large_text);
            }
            if let Some(ref small_image) = assets.small_image {
                discord_assets = discord_assets.small_image(small_image);
            }
            if let Some(ref small_text) = assets.small_text {
                discord_assets = discord_assets.small_text(small_text);
            }
            da = da.assets(discord_assets);
        }

        da
    }
}

impl Drop for RichPresenceClient {
    fn drop(&mut self) {
        if self.connected {
            if let Err(e) = self.disconnect() {
                error!("切断中にエラーが発生: {}", e);
            }
        }
    }
}
