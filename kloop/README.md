# kloop

固定タイムステップのメインループ実装（`FixedLoop`）と、そのための設定（`LoopConfig`）/アプリケーション境界（`App` トレイト）を提供します。時間源には `ktime` の `Clock` を利用します。

## できること（概要）
- `App` トレイト: `update(dt: Duration)` と `render(alpha: f32)` を実装してゲームループに接続。
- `FixedLoop<C: ktime::Clock>`: 時間の進行を監視し、固定Δtで `update` を複数回呼び、補間係数 `alpha∈[0,1)` を計算して `render` を1回呼び出し。
- `TickResult`: 1フレーム中に行った `updates` 回数と `alpha` を取得。
- `LoopConfig`: `from_hz(hz)` で固定Δtを決定し、`with_limits(max_frame_dt, max_updates_per_frame)` でスパイラル回避の制限を設定。
- `run_steps(app, n)`: レンダリング無しで `update` をちょうど `n` 回実行（記録やテストに便利）。

## サンプル

### 例: `kloop_demo`
- 実行（フレーム出力のみ）:
  - `cargo run -p kloop --example kloop_demo`
- 実行（連番PPMから動画も生成、ffmpeg 必須）:
  - `cargo run -p kloop --example kloop_demo -- --video`
- 出力先: `target/examples/kloop_demo/`
  - 連番: `frame_000000.ppm` ～
  - 動画: `out.mp4`（コマンドは `ffmpeg -framerate 60 -i frame_%06d.ppm -c:v libx264 -pix_fmt yuv420p out.mp4`）
