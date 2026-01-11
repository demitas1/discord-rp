# Discord Rich Presence カスタマイザー

Discord Rich Presenceを自由にカスタマイズできるデスクトップアプリケーション。

## 機能

- 任意のテキストやアイコンをDiscordステータスとして表示
- 複数Application IDによるカテゴリ切り替え
- 経過時間の表示
- CLIによる操作
- Windows / macOS / Linux 対応

## 必要条件

- [Rust](https://rustup.rs/) (1.70以上)
- [Discord Developer Portal](https://discord.com/developers/applications) でアプリケーションを作成し、Application IDを取得

## セットアップ

### 1. Discord Developer Portalでアプリケーションを作成

1. [Discord Developer Portal](https://discord.com/developers/applications) にアクセス
2. 「New Application」をクリックしてアプリケーションを作成
   - アプリケーション名がDiscordで「〇〇をプレイ中」として表示されます
   - 例: "Working", "Studying", "Playing" など用途別に複数作成可能
3. 「General Information」から **Application ID** をコピー

### 2. 環境変数を設定

```bash
# .envファイルを作成（複数ID対応）
cat << 'EOF' > .env
DISCORD_APPLICATION_ID_1=111111111111111111
DISCORD_APPLICATION_ID_2=222222222222222222
DISCORD_APPLICATION_ID_3=333333333333333333
EOF
```

インデックス番号（1, 2, 3...）でアプリケーションを切り替えます。

### 3. ビルド

```bash
cd discord-rp
cargo build --release
```

## CLIの使い方

### 基本コマンド

```bash
# 開発時（cargo経由）
cargo run -p rp-cli -- --help
cargo run -p rp-cli -- list
cargo run -p rp-cli -- test
cargo run -p rp-cli -- set -d "テキスト" -s "状態"

# ビルド後（バイナリ直接実行）
./target/release/discord-rp --help
./target/release/discord-rp list
./target/release/discord-rp test
./target/release/discord-rp set -d "テキスト" -s "状態"
```

以降の例では簡潔のため `discord-rp` と表記しますが、開発時は `cargo run -p rp-cli --` に置き換えてください。

```bash
# 登録済みApplication ID一覧を表示
discord-rp list

# 接続テスト（デフォルト: インデックス1）
discord-rp test

# 接続テスト（インデックス指定）
discord-rp -i 2 test

# Rich Presenceを設定
discord-rp set -d "テキスト" -s "状態"

# 別のApplication IDで設定（インデックス2）
discord-rp -i 2 set -d "勉強中" -s "数学"
```

### グローバルオプション

| オプション | 説明 |
|-----------|------|
| `-i, --index <N>` | 使用するApplication IDのインデックス（1始まり、デフォルト: 1） |
| `-c, --config <PATH>` | 設定ファイルのパス |
| `--log-level <LEVEL>` | ログレベル（trace/debug/info/warn/error） |

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
# カテゴリ1（例: "Working"）で作業状態を表示
discord-rp -i 1 set -d "Rustプロジェクト" -s "Phase 1" --elapsed

# カテゴリ2（例: "Studying"）で勉強状態を表示
discord-rp -i 2 set -d "数学" -s "線形代数"

# カテゴリ3（例: "Playing"）でゲーム状態を表示
discord-rp -i 3 set -d "Minecraft" --elapsed

# 30秒間だけ表示
discord-rp set -d "休憩中" -D 30

# 画像付きで表示（Developer Portalで画像を登録済みの場合）
discord-rp set -d "作業中" --large-image "my-icon" --large-text "カスタムアイコン"
```

## 複数Application IDの活用

Discord Developer Portalで用途別にアプリケーションを作成することで、ステータスのカテゴリを切り替えられます。

| インデックス | アプリケーション名（例） | 用途 |
|-------------|------------------------|------|
| 1 | Working | 仕事・作業 |
| 2 | Studying | 勉強 |
| 3 | Playing | ゲーム |

アプリケーション名は Discord Developer Portal の「General Information」→「NAME」で変更できます。

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
