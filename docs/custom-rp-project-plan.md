# Custom Rich Presence アプリケーション開発計画書

## 概要

Discord Rich Presenceを自由にカスタマイズできるデスクトップアプリケーションを開発する。ユーザーが任意のテキストやアイコンをDiscordのステータスとして表示できるようにする。

## 目標

- ユーザーが簡単にDiscord Rich Presenceをカスタマイズできるツールを提供
- Windows / macOS / Linux でのクロスプラットフォーム対応
- 軽量で常駐可能なアプリケーション

## 技術スタック

| レイヤー | 技術 |
|---------|------|
| コアロジック | Rust |
| GUI フレームワーク | Tauri v2 |
| フロントエンド | React + TypeScript |
| Discord連携 | discord-rich-presence crate |

## プロジェクト構成

```
discord-custom-rp/
├── crates/
│   ├── rp-core/          # コアライブラリ
│   └── rp-cli/           # CLIバイナリ
├── src-tauri/            # Tauri バックエンド
├── src/                  # React フロントエンド
├── Cargo.toml            # ワークスペース定義
└── package.json
```

## 開発フェーズ

### Phase 1: コアライブラリ + CLI

**目的**: Rich Presenceの基本機能を実装し、動作検証を行う

**成果物**:
- `rp-core`: Discord IPC接続、アクティビティ更新のライブラリ
- `rp-cli`: コマンドラインから操作可能なバイナリ

**主要機能**:
- Discord IPCへの接続/切断
- アクティビティの設定（details, state, timestamps, assets）
- 接続状態の監視と自動再接続
- 設定ファイル（TOML）の読み書き

**検証項目**:
- Windows / macOS / Linux での動作確認
- Discordクライアント未起動時のエラーハンドリング
- 長時間稼働時の安定性

### Phase 2: Tauri GUI アプリケーション

**目的**: 一般ユーザーが使いやすいGUIを提供する

**成果物**:
- Tauriベースのデスクトップアプリケーション
- Reactによるフロントエンド

**主要機能**:
- プレゼンス設定のフォーム入力
- プリセット保存/読み込み
- システムトレイ常駐
- 接続状態のリアルタイム表示
- 起動時の自動接続オプション

**UI構成案**:
- メインウィンドウ: 設定フォーム + プレビュー
- システムトレイ: クイックアクセスメニュー

### Phase 3: 拡張機能（オプション）

**検討中の追加機能**:
- 複数プロファイルの切り替え
- タイムスケジュールによる自動切り替え
- Discord Developer Portal連携（Application ID管理）
- 外部API連携（Webhook受信でステータス更新）

## Discord Developer Portal 設定

アプリケーション利用には、ユーザーが自身のDiscord Applicationを作成する必要がある。

**必要な設定**:
1. Application作成（Application ID取得）
2. Rich Presenceアセット画像のアップロード（任意）

**アプリ内での案内**:
- 初回起動時にセットアップガイドを表示
- Developer Portalへのリンクを提供

## 配布形態

| プラットフォーム | 形式 |
|-----------------|------|
| Windows | .msi / .exe (NSIS) |
| macOS | .dmg / .app |
| Linux | .deb / .AppImage |

## ライセンス

MIT License（予定）

## 備考

- Discord利用規約への準拠を確認すること
- Rate Limit（15秒以上の更新間隔）を遵守する実装とする
