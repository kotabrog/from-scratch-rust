# kimgfmt

画像フォーマットの最小実装（まずは保存専用）。std のみで、まず PPM(P6) を提供します。

## 提供機能（現状）
- PPM(P6) 書き出し（RGBA8 リトルエンディアンの `u32` 配列から RGB を出力）
  - `write_ppm_from_rgba_le(pixels, width, height, path)`
  - `write_ppm_from_rgba_le_to_writer(pixels, width, height, writer)`
- BMP(24-bit, BI_RGB) 書き出し（BGR順、行は4バイト境界にパディング、Top-Down）
  - `bmp::write_bmp24_from_rgba_le(pixels, width, height, path)`
  - `bmp::write_bmp24_from_rgba_le_to_writer(pixels, width, height, writer)`
- 共通API（フォーマット選択）
  - `save_rgba_le(pixels, width, height, path, Format::Ppm|Bmp24)`
  - `save_rgba_le_to_writer(pixels, width, height, Format::..., writer)`

ピクセル契約:
- 行優先（row-major）、原点は左上 `(0,0)`
- 1ピクセルは RGBA8 を little-endian で `u32` に格納（`u32::to_le_bytes()` が `[r,g,b,a]`）
- PPM は alpha を無視して RGB のみを書き出します

## 使用例
```rust
use kimgfmt::{write_ppm_from_rgba_le, save_rgba_le, Format};

let w = 2usize;
let h = 1usize;
let pixels = [
    u32::from_le_bytes([10, 20, 30, 255]),
    u32::from_le_bytes([40, 50, 60, 128]),
];
write_ppm_from_rgba_le(&pixels, w, h, "out.ppm").unwrap();
// or using the common API
save_rgba_le(&pixels, w, h, "out.bmp", Format::Bmp24).unwrap();
```

## 今後の拡張
- `Options`（alpha の扱い、top-down/bottom-up など）
- 読み込み（decode）は段階的に対応予定
