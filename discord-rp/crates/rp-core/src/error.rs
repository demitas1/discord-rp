//! エラー型の定義

use thiserror::Error;

/// Rich Presenceクライアントのエラー型
#[derive(Debug, Error)]
pub enum Error {
    /// Discord IPCへの接続に失敗
    #[error("Discord への接続に失敗しました: {0}")]
    ConnectionFailed(String),

    /// Discordクライアントが起動していない
    #[error("Discord が起動していません")]
    DiscordNotRunning,

    /// アクティビティの更新に失敗
    #[error("アクティビティの更新に失敗しました: {0}")]
    ActivityUpdateFailed(String),

    /// 設定ファイルの読み込みに失敗
    #[error("設定ファイルの読み込みに失敗しました: {0}")]
    ConfigLoadFailed(String),

    /// 設定ファイルの保存に失敗
    #[error("設定ファイルの保存に失敗しました: {0}")]
    ConfigSaveFailed(String),

    /// 無効なApplication ID
    #[error("無効な Application ID です: {0}")]
    InvalidApplicationId(String),

    /// 接続が切断された
    #[error("Discord との接続が切断されました")]
    Disconnected,

    /// Rate limit超過
    #[error("更新頻度が高すぎます。15秒以上の間隔を空けてください")]
    RateLimited,
}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, Error>;
