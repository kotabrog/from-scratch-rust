# kplatform_term

ターミナル（ANSI/VT100）を簡易プラットフォームとして扱うバックエンドです。オルタネートスクリーン上に
`kpix::Surface` 由来のピクセルを TrueColor で描画し、キー入力（生の ESC シーケンス）を最小限扱います。

## できること（概要）
- 描画: `present_rgba_le(width, height, &pixels)`（RGBA little-endian）
  - 背景色エスケープ（`ESC[48;2;R;G;Bm`）+ 空白で粗いピクセル表示
  - 行ごとのランレングス最適化（色の連続をまとめて出力）
- 入力: `poll_event()`
  - キー: `Esc` / `Enter` / `Backspace` / 矢印（←→↑↓）/ ASCII 文字（`Key::Char`）
  - 端末の性質上 `KeyUp` は基本的に来ません（`KeyDown` のみ）
- 端末制御: オルタネートスクリーンへの切替・復帰、カーソル非表示・表示（Dropで確実に復帰）

## 規約 / 注意
- 端末: ANSI TrueColor（24bit色）対応の端末を想定
- 表示: 1セル=1ピクセル相当の粗い表示。高解像表示や半角/全角は考慮しません
- 入力: UTF-8 多バイトや IME は未対応（今後拡張可能）

## サンプル

### グラデーション（簡易アニメ）
- 実行: `cargo run -p kplatform_term --example term_gradient`
- オプション: `--seconds <f32>`（表示時間。既定は約4秒）
- 操作: `q` で終了

### キー入力の確認
- 実行: `cargo run -p kplatform_term --example keys`
- オプション: `--seconds <f32>`（収集時間。既定は10秒）
- 操作: `q` で終了。終了後、取得したイベントの一覧を表示
