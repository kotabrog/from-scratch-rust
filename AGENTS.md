# Agent Guidelines

このドキュメントは、本リポジトリでのテスト作成・配置ルールをまとめたものです。

## 基本方針
- 小さく独立したユニットテストを基本とし、挙動の意図を明確にする。
- 実装の公開 API に沿った順序でテストを並べ、読みやすさと保守性を高める。
- 浮動小数の比較は厳密等号を避け、近似比較を徹底する。

## テストの配置と順序
- `#[cfg(test)] mod tests { ... }` を基本に、同一ソースファイル内にユニットテストを配置する。
- テストの並び順は「実装している関数・メソッドの順番」に合わせる。
  - 例: `new` → `dot` → `length/length_squared` → `try_normalize/normalize/normalize_or_zero` → `distance/distance_squared` → `lerp` → 演算子系（`Add/Sub/Mul/Div` など）。
- ファイル内で閉じている“統合テスト的”なテスト（複数の API をまたいで検証するもの）は最後に配置する。
  - テスト名に `integration_` プレフィックスを付け、意図を明確にする。

## テストの粒度
- 原則、1テストで1つの関数・メソッドの挙動を検証する。
  - 複数関数をまたぐ検証は「統合テスト枠」に分離する。
- 同一関数の正常系・境界値・異常系は、テストケースを分けて記述する。

## 浮動小数の扱い（kmath）
- `kmath::num` のユーティリティを使用する。
  - 近似比較: `approx_eq`, `approx_eq_with`（必要に応じて許容誤差を指定）。
  - ゼロ判定: `is_zero` を使用（ゼロ除算の回避には `safe_div`）。
  - 直接の `==` 比較は、0.0 など数学的に厳密な値を除き避ける。

## CI/実行
- Makefile ターゲットを利用する。
  - フォーマット検証: `make fmt-check`
  - Lint: `make lint`（Clippy 警告・rustc 警告をエラー化）
  - テスト: `make test`
  - CI まとめ: `make ci`
- GitHub Actions は `main` への Pull Request 時に `lint` と `test` を実行する。

## 開発運用（examples の出力先）
- 出力ディレクトリは `target/` 配下に統一する。
  - 既定: `<CARGO_TARGET_DIR|./target>/examples/<example-name>/...`
  - 例の生成物（PPM/BMP など）は上記ディレクトリに吐き出すこと。
- 生成物をリポジトリ直下に置かない（`.gitignore` 管理外の散乱防止）。
- 解決ヘルパ: `kdev` クレートの `kdev::out::example_output_dir("<example>")` を利用する。
  - `CARGO_TARGET_DIR` が設定されていればそれを尊重、未設定なら `./target` を使用。
  - 各クレートの examples から利用する場合は、`kdev` を通常依存ではなく **dev-dependencies** として追加する。
    - 例）`[dev-dependencies] kdev = { path = "../kdev" }`
  - 典型コード:
    - `let out_dir = kdev::out::example_output_dir("shapes")?;`
    - `io::write_ppm(&surface, out_dir.join("shapes.ppm"))?;`
