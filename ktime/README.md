# ktime

時間ユーティリティ集。時間源の抽象化（`Clock`）、`Duration` の補助関数、簡易 `Stopwatch` を提供します。

## できること（概要）
- `Clock` トレイト: `now() -> Instant` を提供する時間源の抽象化。
  - `SystemClock`: 実時間の `Instant::now()` を返す実装。
  - `FakeClock`: 手動で `advance(dt)` できる擬似時計。テストやヘッドレス実行に便利。
- `duration` ユーティリティ:
  - `from_secs_f64` / `from_secs_f32`: 秒から `Duration` を生成（負は0、非常に大きい値はオーバーフロー防止）。
  - `secs_f64` / `secs_f32`: `Duration` を秒に変換。
- `Stopwatch`: `start`/`stop`/`reset`/`is_running`/`elapsed` を持つ簡易ストップウォッチ。
