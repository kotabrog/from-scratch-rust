# kimgfmt

画像フォーマットの最小実装（保存専用）。PPM/BMP の書き出しを提供します。

## できること（概要）
- PPM(P6) 書き出し: RGBA8 little-endian の `u32` 配列から RGB を出力
  - `ppm::write_ppm_from_rgba_le` / `ppm::write_ppm_from_rgba_le_to_writer`
- BMP 24-bit (BI_RGB, BGR) 書き出し: 行は 4 バイト境界にパディング、Top-Down（負の高さ）
  - `bmp::write_bmp24_from_rgba_le` / `bmp::write_bmp24_from_rgba_le_to_writer`
- 共通API（フォーマット選択）
  - `save_rgba_le` / `save_rgba_le_to_writer`（`Format::{Ppm, Bmp24}`）

## 規約
- ピクセル契約: 行優先（row-major）、原点は左上 `(0,0)`、1ピクセルは RGBA8 を little-endian の `u32` に格納
  - `u32::to_le_bytes() -> [r, g, b, a]`
- アルファ: 書き出し時は無視（RGB のみを出力）
- オリエンテーション: Top-Down 想定（BMP は高さを負で記録）
