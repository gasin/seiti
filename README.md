# 最適整地 (Seiti)

19×19の囲碁盤面を生成し、整数計画法を用いて最適な整地パターンを探索するアプリケーションです。

## 機能

- **盤面生成**: Perlinノイズを用いた自然な盤面生成
- **整地アルゴリズム**: 整数計画法（IP）による最適整地パターンの探索
- **石の移動アニメーション**: ハンガリアン法による移動距離最小化を行い、整地前後の石の移動を可視化

## 技術スタック

### バックエンド
- **Rust**: コアロジックとAPIサーバ
- **axum**: Webフレームワーク
- **good_lp + HiGHS**: 整数計画法ソルバ

### フロントエンド
- **React + TypeScript**: UIフレームワーク
- **Vite**: ビルドツール

## セットアップ

```bash
# 依存関係のインストール
npm run frontend:install

# Rustの依存関係は初回ビルド時に自動的にダウンロードされます
```

## 開発

バックエンドとフロントエンドを別々のターミナルで起動します。

### バックエンドの起動

```bash
npm run dev:backend
# または
cargo run -p backend
```

バックエンドは `http://127.0.0.1:3000` で起動します。

### フロントエンドの起動

```bash
npm run dev:frontend
# または
npm --prefix frontend run dev
```

フロントエンドは `http://localhost:5173` (Viteのデフォルトポート) で起動します。

フロントエンドは `frontend/vite.config.ts` の proxy 設定により `/api` リクエストをバックエンドへ転送します。

## ビルド

```bash
# 全体をビルド
npm run build

# 個別にビルド
npm run build:backend
npm run build:frontend
```

## プロジェクト構成

```
seiti/
├── core/          # 共通ロジック（純Rust）
│   ├── src/
│   │   ├── generate.rs    # 盤面生成
│   │   ├── level/         # 整地アルゴリズム
│   │   ├── matching.rs    # 石の移動計算（ハンガリアン法）
│   │   └── ...
│   └── Cargo.toml
├── backend/       # Rust APIサーバ（axum）
│   ├── src/main.rs
│   └── Cargo.toml
├── frontend/      # React + TypeScript
│   ├── src/
│   └── package.json
└── Cargo.toml     # ワークスペース設定
```

## API仕様

### `POST /api/board/generate`

盤面を生成します。

**リクエスト:**
```json
{
  "seed": 1
}
```

**レスポンス:**
```json
{
  "size": 19,
  "seed": 1,
  "stones": [0, 1, 2, ...],  // 0=空, 1=黒石, 2=白石
  "territory": [0, 1, 2, ...] // 0=なし, 1=黒地, 2=白地
}
```

### `POST /api/board/level`

盤面を整地します。

**リクエスト:**
```json
{
  "board": { ... }  // BoardState
}
```

**レスポンス:**
```json
{
  "board": { ... },  // 整地後のBoardState
  "moves": [         // 石の移動情報
    {
      "color": 1,           // 1=黒, 2=白
      "from": [0, 0],       // [x, y]
      "to": [1, 1]          // [x, y]
    },
    ...
  ]
}
```

### `GET /health`

ヘルスチェックエンドポイント。`"ok"`を返します。

## ライセンス

MIT License
