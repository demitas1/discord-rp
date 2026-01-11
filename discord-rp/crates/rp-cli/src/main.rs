//! Discord Rich Presence CLI ツール

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rp_core::{Activity, Config, RichPresenceClient};
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "discord-rp")]
#[command(about = "Discord Rich Presence をカスタマイズするCLIツール")]
#[command(version)]
struct Cli {
    /// 使用するApplication IDのインデックス（1始まり、デフォルト: 1）
    #[arg(short, long, default_value = "1")]
    index: u32,

    /// 設定ファイルのパス
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// ログレベル（trace, debug, info, warn, error）
    #[arg(long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Rich Presenceを設定して表示
    Set {
        /// 詳細テキスト（1行目）
        #[arg(short, long)]
        details: Option<String>,

        /// 状態テキスト（2行目）
        #[arg(short, long)]
        state: Option<String>,

        /// 経過時間を表示
        #[arg(long)]
        elapsed: bool,

        /// 大きい画像のキー
        #[arg(long)]
        large_image: Option<String>,

        /// 大きい画像のツールチップ
        #[arg(long)]
        large_text: Option<String>,

        /// 小さい画像のキー
        #[arg(long)]
        small_image: Option<String>,

        /// 小さい画像のツールチップ
        #[arg(long)]
        small_text: Option<String>,

        /// 表示を維持する秒数（指定しない場合は Ctrl+C まで維持）
        #[arg(short = 'D', long)]
        duration: Option<u64>,
    },

    /// Rich Presenceをクリア
    Clear,

    /// 接続テスト
    Test,

    /// 登録済みApplication ID一覧を表示
    List,

    /// 設定ファイルを生成
    Init {
        /// 出力先パス（指定しない場合はデフォルトパス）
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    // .env ファイルを読み込む（存在しなくてもエラーにしない）
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    // ロギングの初期化
    let filter =
        EnvFilter::try_new(&cli.log_level).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // 環境変数から設定を読み込む
    let config = Config::from_env();

    match cli.command {
        Commands::Set {
            ref details,
            ref state,
            elapsed,
            ref large_image,
            ref large_text,
            ref small_image,
            ref small_text,
            duration,
        } => {
            let app_id = get_app_id(&config, cli.index)?;
            cmd_set(
                &app_id,
                cli.index,
                details.clone(),
                state.clone(),
                elapsed,
                large_image.clone(),
                large_text.clone(),
                small_image.clone(),
                small_text.clone(),
                duration,
            )
        }
        Commands::Clear => {
            let app_id = get_app_id(&config, cli.index)?;
            cmd_clear(&app_id)
        }
        Commands::Test => {
            let app_id = get_app_id(&config, cli.index)?;
            cmd_test(&app_id, cli.index)
        }
        Commands::List => cmd_list(&config),
        Commands::Init { output } => cmd_init(output),
    }
}

/// Application IDを取得
fn get_app_id(config: &Config, index: u32) -> Result<String> {
    config
        .get_application_id(index)
        .map(|s| s.to_string())
        .map_err(|e| anyhow::anyhow!("{}", e))
}

/// setコマンドの実行
fn cmd_set(
    app_id: &str,
    index: u32,
    details: Option<String>,
    state: Option<String>,
    elapsed: bool,
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
    duration: Option<u64>,
) -> Result<()> {
    let mut client =
        RichPresenceClient::new(app_id).context("クライアントの作成に失敗しました")?;

    client.connect().context("Discord への接続に失敗しました")?;

    let mut activity = Activity::new();

    if let Some(d) = details {
        activity = activity.details(d);
    }
    if let Some(s) = state {
        activity = activity.state(s);
    }
    if elapsed {
        activity = activity.start_timestamp();
    }
    if let Some(key) = large_image {
        activity = activity.large_image(key, large_text);
    }
    if let Some(key) = small_image {
        activity = activity.small_image(key, small_text);
    }

    client
        .update_activity(&activity)
        .context("アクティビティの更新に失敗しました")?;

    info!("Rich Presence を設定しました");
    println!(
        "Rich Presence を設定しました（インデックス: {}）。Ctrl+C で終了します。",
        index
    );

    // 指定時間または Ctrl+C まで待機
    if let Some(secs) = duration {
        std::thread::sleep(std::time::Duration::from_secs(secs));
    } else {
        // Ctrl+C を待機
        let (tx, rx) = std::sync::mpsc::channel();
        ctrlc::set_handler(move || {
            let _ = tx.send(());
        })
        .context("Ctrl+C ハンドラの設定に失敗しました")?;
        rx.recv().ok();
    }

    client.disconnect()?;
    println!("終了しました。");
    Ok(())
}

/// clearコマンドの実行
fn cmd_clear(app_id: &str) -> Result<()> {
    let mut client =
        RichPresenceClient::new(app_id).context("クライアントの作成に失敗しました")?;

    client.connect().context("Discord への接続に失敗しました")?;

    client
        .clear_activity()
        .context("アクティビティのクリアに失敗しました")?;

    println!("Rich Presence をクリアしました。");
    Ok(())
}

/// testコマンドの実行
fn cmd_test(app_id: &str, index: u32) -> Result<()> {
    println!("Discord への接続をテストしています...");
    println!("インデックス: {}", index);
    println!("Application ID: {}", app_id);

    let mut client =
        RichPresenceClient::new(app_id).context("クライアントの作成に失敗しました")?;

    match client.connect() {
        Ok(()) => {
            println!("✓ Discord に接続しました");
            client.disconnect()?;
            println!("✓ 接続テスト成功");
            Ok(())
        }
        Err(e) => {
            error!("接続テスト失敗: {}", e);
            anyhow::bail!("接続テスト失敗: {}", e)
        }
    }
}

/// listコマンドの実行
fn cmd_list(config: &Config) -> Result<()> {
    let indices = config.registered_indices();

    if indices.is_empty() {
        println!("登録されているApplication IDがありません。");
        println!();
        println!("環境変数で設定してください:");
        println!("  DISCORD_APPLICATION_ID_1=<Application ID>");
        println!("  DISCORD_APPLICATION_ID_2=<Application ID>");
        println!("  ...");
        return Ok(());
    }

    println!("登録済みApplication ID:");
    println!();
    for idx in indices {
        if let Ok(app_id) = config.get_application_id(idx) {
            // Application IDの一部を隠す
            let masked = if app_id.len() > 8 {
                format!("{}...{}", &app_id[..4], &app_id[app_id.len() - 4..])
            } else {
                app_id.to_string()
            };
            println!("  [{}] {}", idx, masked);
        }
    }
    println!();
    println!("使用例: discord-rp -i 1 set -d \"作業中\"");

    Ok(())
}

/// initコマンドの実行
fn cmd_init(output: Option<PathBuf>) -> Result<()> {
    let path = output
        .or_else(Config::default_path)
        .context("設定ファイルのパスを決定できませんでした")?;

    let config = Config::new();
    config.save(&path)?;

    println!("設定ファイルを作成しました: {}", path.display());
    println!();
    println!("Application IDは環境変数で設定してください:");
    println!("  DISCORD_APPLICATION_ID_1=<Application ID>");
    println!("  DISCORD_APPLICATION_ID_2=<Application ID>");
    println!("  ...");
    Ok(())
}
