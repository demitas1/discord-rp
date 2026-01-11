# Discord Rich Presence カスタマイザー

Discord Rich Presenceを自由にカスタマイズできるデスクトップアプリケーション。

## 機能

- 任意のテキストやアイコンをDiscordステータスとして表示
- 経過時間の表示
- 設定ファイル（TOML）による管理
- CLIによる操作
- Windows / macOS / Linux 対応

## 必要条件

- [Rust](https://rustup.rs/) (1.70以上)
- [Discord Developer Portal](https://discord.com/developers/applications) でアプリケーションを作成し、Application IDを取得

## セットアップ

### 1. Discord Developer Portalでアプリケーションを作成

1. [Discord Developer Portal](https://discord.com/developers/applications) にアクセス
2. 「New Application」をクリックしてアプリケーションを作成
3. 「General Information」から **Application ID** をコピー

### 2. 環境変数を設定

```bash
# .envファイルを作成
echo "DISCORD_APPLICATION_ID=あなたのApplication ID" > .env
```

または、コマンド実行時に `--app-id` オプションで指定することもできます。

### 3. ビルド

```bash
cd discord-rp
cargo build --release
```

## CLIの使い方

### 基本コマンド

```bash
# ヘルプを表示
cargo run -p rp-cli -- --help

# 接続テスト
cargo run -p rp-cli -- test

# Rich Presenceを設定
cargo run -p rp-cli -- set --details "テキスト" --state "状態"

# Rich Presenceをクリア
cargo run -p rp-cli -- clear

# 設定ファイルを生成
cargo run -p rp-cli -- init
```

### setコマンドのオプション

| オプション | 説明 |
|-----------|------|
| `-d, --details <TEXT>` | 詳細テキスト（1行目） |
| `-s, --state <TEXT>` | 状態テキスト（2行目） |
| `--elapsed` | 経過時間を表示 |
| `--large-image <KEY>` | 大きい画像のキー |
| `--large-text <TEXT>` | 大きい画像のツールチップ |
| `--small-image <KEY>` | 小さい画像のキー |
| `--small-text <TEXT>` | 小さい画像のツールチップ |
| `-D, --duration <SECS>` | 表示を維持する秒数 |

### 使用例

```bash
# シンプルなステータス表示
cargo run -p rp-cli -- set -d "コーディング中" -s "Rustプロジェクト"

# 経過時間付きで表示
cargo run -p rp-cli -- set -d "ゲームをプレイ中" --elapsed

# 30秒間だけ表示
cargo run -p rp-cli -- set -d "休憩中" -D 30

# 画像付きで表示（Developer Portalで画像を登録済みの場合）
cargo run -p rp-cli -- set -d "作業中" --large-image "my-icon" --large-text "カスタムアイコン"
```

### グローバルオプション

| オプション | 説明 |
|-----------|------|
| `-a, --app-id <ID>` | Application ID（環境変数でも指定可） |
| `-c, --config <PATH>` | 設定ファイルのパス |
| `--log-level <LEVEL>` | ログレベル（trace/debug/info/warn/error） |

## 設定ファイル

`discord-rp init` で設定ファイルを生成できます。

```toml
# ~/.config/discord-rp/config.toml

application_id = "あなたのApplication ID"
auto_connect = true
auto_reconnect = true
reconnect_interval = 30

[activity]
details = "デフォルトのテキスト"
state = "デフォルトの状態"
```

## プロジェクト構成

```
discord-rp/
├── Cargo.toml              # ワークスペース定義
└── crates/
    ├── rp-core/            # コアライブラリ
    │   └── src/
    │       ├── lib.rs
    │       ├── activity.rs # アクティビティ定義
    │       ├── client.rs   # Discord IPCクライアント
    │       ├── config.rs   # 設定管理
    │       └── error.rs    # エラー型
    └── rp-cli/             # CLIバイナリ
        └── src/
            └── main.rs
```

## Discordクライアントの設定

Rich Presenceを表示するには、Discordクライアント側の設定が必要です。

### 1. アクティビティステータスを有効にする

1. Discordを開く
2. 左下の **歯車アイコン**（ユーザー設定）をクリック
3. 左メニューから **アクティビティのプライバシー** を選択
4. 「現在のアクティビティをステータスに表示する」を **オン** にする

### 2. 表示されない場合のチェックリスト

| 確認項目 | 対処法 |
|---------|--------|
| Discordが起動していない | Discordデスクトップアプリを起動する |
| ブラウザ版Discordを使用 | デスクトップアプリを使用する（ブラウザ版はIPC非対応） |
| アクティビティ設定がオフ | 上記の手順で有効にする |
| ステータスがオフライン | オンライン/取り込み中/退席中に変更する |

## 注意事項

- Discordデスクトップアプリが起動している必要があります（ブラウザ版は非対応）
- アクティビティの更新は15秒以上の間隔を空ける必要があります（Discord API制限）
- 画像を表示するには、事前にDiscord Developer Portalでアセットを登録してください

## ライセンス

MIT License
