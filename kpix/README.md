# kpix

最小構成のピクセル描画ライブラリ。CPU上のピクセルバッファに基本図形を描き、PPM/BMPで保存。

## できること（概要）
- 低レベル: `Surface`/`Color` によるピクセルバッファ管理（RGBA を `u32` に格納）。
- 描画: `clear` と `set_pixel`（クリップは暗黙）。
- 線分: `draw::draw_line`（Bresenham、端点含む）。
- 矩形: `draw::draw_rect`（外周）/`draw::fill_rect`（塗りつぶし）。負サイズ正規化・クリップ対応。
- 円: `draw::draw_circle`（ミッドポイント法、`r=0` は中心のみ）。
- 出力: `io::write_ppm` による PPM(P6) 保存（alpha は無視）、`io::write_bmp` による BMP(24-bit, BGR, BI_RGB, top-down) 保存。
  - 保存処理は内部で `kimgfmt` に委譲しています。将来的には `kimgfmt` の直接利用を推奨します。

## 規約
- 座標系: 原点は左上 `(0,0)`、xは右が正、yは下が正。
- ピクセル表現: `u32` に little-endian の RGBA を格納（`u32::from_le_bytes([r,g,b,a])`）。
- 範囲外アクセス: `set_pixel` はクリップ（何もしない）。

## サンプル

### グラデーション
- 実行: `cargo run -p kpix --example gradient -- gradient.ppm`
- 出力: 256x256 のグラデーション画像（`gradient.ppm` と `gradient.bmp` を出力）

### 図形（線・矩形・円）
- 実行: `cargo run -p kpix --example shapes`
- 出力: `shapes.ppm` と `shapes.bmp`（グリッド＋スター状の線／矩形の枠と塗りつぶし／同心円）
