# kraster2d

CPU 上で動作する最小構成の 2D 三角形ラスタライザです。`kpix::Surface` をフレームバッファとして利用し、
ソリッド塗り、頂点カラー補間、テクスチャ（最近傍）描画をサポートします。画像の保存は `kimgfmt` に委譲します。

## できること（概要）
- フレーム: `core::frame::Frame`（カラーのみ。深度は将来予約）
- テクスチャ: `core::texture::Texture`（RGBA8 little-endian、uv∈[0,1]、最近傍サンプリング、クランプ）
- 三角形ラスタ（Top-Left ルール・バリセントリック）:
  - `raster::draw_triangle_solid`
  - `raster::draw_triangle_vertex_color`
  - `raster::draw_triangle_textured`
  - 頂点型: `raster::Vertex { pos: Vec3, uv: Vec2, color: [f32; 3] }`
- 出力: `io::write::{write_ppm, write_bmp}`（内部で `kimgfmt` を利用）

## 規約（Conventions）
- 座標系: 画面座標、原点は左上 `(0,0)`、x は右が正、y は下が正（`kpix` と同じ）
- サンプリング点: ピクセル中心の `(x + 0.5, y + 0.5)` を評価
- エッジ判定: Top-Left ルール（上辺/左辺を内側に含み、右/下辺は除外）で“継ぎ目の穴”を防止
- バウンディングボックス: `floor(min)` と `ceil(max)-1` を採用し、フレーム範囲にクランプ
- 重み: エッジ関数 `E(a,b,p)` による符号付き面積からバリセントリック重みを算出し正規化（a+b+c ≈ 1）
- 色・ピクセル: RGBA8 を `u32` little-endian に格納（アルファは現状未使用）
- テクスチャ: uv は [0,1] を想定しクランプ、原点は左上（v は下に増加）、フィルタは最近傍

## サンプル

### 描画確認
- 実行: `cargo run -p kraster2d --example solid`
- 出力: `target/examples/solid/frame0000.ppm`

### トライアングル
- 実行: `cargo run -p kraster2d --example tri_solid`
- 出力: 三角形（`target/examples/tri_solid/tri_solid.ppm`）

### テクスチャと回転
- 実行: `cargo run -p kraster2d --example rotating_quad`
- 出力: テクスチャ適用と図形の回転（`target/examples/rotating_quad/frame0000.ppm`（連番））
