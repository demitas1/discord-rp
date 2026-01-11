//! Discord Rich Presence コアライブラリ
//!
//! Discord IPCへの接続、アクティビティ更新、設定管理を提供する

mod activity;
mod client;
mod config;
mod error;

pub use activity::{Activity, ActivityAssets, ActivityTimestamps};
pub use client::RichPresenceClient;
pub use config::Config;
pub use error::{Error, Result};
