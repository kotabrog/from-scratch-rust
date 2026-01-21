# kpix

最小構成のピクセル描画ライブラリ。CPU上のピクセルバッファに基本図形を描き、まずは **PPM** で保存します（依存なし）。

## 仕様と前提
- 座標系: 原点は左上 `(0,0)`、xは右が正、yは下が正。
- ピクセル表現: `u32` に little-endian の RGBA を格納（`u32::from_le_bytes([r,g,b,a])`）。
- 範囲外アクセス: `set_pixel` はクリップ（何もしない）。

## 主なAPI
- `Color { r,g,b,a }` と `Color::rgba(r,g,b,a)`
- `Surface::new(w,h)` / `width()` / `height()`
- `Surface::clear(color)` / `Surface::set_pixel(x,y,color)`
- `io::write_ppm(&surface, path)`（P6形式、alphaは無視）

## 例（グラデーション）
```bash
cargo run -p kpix --example gradient -- gradient.ppm
```
`examples/gradient.rs` は 256x256 のグラデーションを生成し、PPM で保存します。

## 今後の拡張（予定）
- `draw_line`（Bresenham）、`draw_rect` / `fill_rect`、`draw_circle`
- PNG出力（依存を許容する場合）
- `blend_pixel`（straight alpha: src-over）

