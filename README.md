# Discord Rich Presence カスタマイザー

Discord Rich Presenceを自由にカスタマイズできるデスクトップアプリケーション。

## 機能

- 任意のテキストやアイコンをDiscordステータスとして表示
- プリセットの保存・読み込み
- システムトレイ常駐
- Windows / macOS / Linux 対応

## 必要条件

- [Rust](https://rustup.rs/) (1.70以上)
- [Node.js](https://nodejs.org/) (18以上)
- [Discord Developer Portal](https://discord.com/developers/applications) でアプリケーションを作成し、Application IDを取得

## セットアップ

```bash
# 依存関係のインストール
npm install

# 開発モードで起動
npm run tauri dev

# プロダクションビルド
npm run tauri build
```

## CLIの使用

```bash
# CLIのビルドと実行
cargo run -p rp-cli
```

## ライセンス

MIT License
