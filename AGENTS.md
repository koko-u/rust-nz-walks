# AGENTS.md

このファイルは、このリポジトリで Rust + Axum のコードを変更する際のルールを定める。

## 基本方針

- Rust 2024 edition と、ワークスペースの `Cargo.toml` に定義された依存関係・バージョンを使用する。
- 変更は要求された範囲に限定し、既存の公開 API、レスポンス形式、DB スキーマを理由なく壊さない。
- 新しい依存クレートは、標準ライブラリや既存依存で解決できない場合にのみ追加する。共通の依存はルートの `[workspace.dependencies]` に定義し、各 crate から `workspace = true` で参照する。
- `unsafe`、`unwrap()`、`expect()`、`panic!()` をアプリケーションの通常の制御フローに使用しない。失敗は型付きの `Result` と `?` で伝播する。
- 秘密情報、認証情報、接続文字列をソース、ログ、エラーレスポンスへ出力しない。秘密値には `secrecy` を使用する。
- コメントは「何をしているか」ではなく、設計上の理由や制約を説明する場合にだけ追加する。

## プロジェクト構成

- API 実装は `api` crate に置く。
- 機能は `api/src/features/<feature>/` にまとめ、既存の Regions feature に合わせて責務を分ける。
  - `routes`: URL と HTTP method を handler に接続する。
  - `handlers`: extractor、入力検証、トランザクション境界、HTTP レスポンスへの変換を扱う。
  - `requests`: HTTP 入力 DTO と `garde` の検証規則を定義する。
  - `commands`: handler から repository へ渡す書き込み用データを定義する。
  - `repositories`: SQLx による永続化だけを扱い、HTTP 型へ依存しない。
  - `rows`: DB の問い合わせ結果を表す。
  - `models`: API が返すドメイン／レスポンスモデルを表す。
- 新しい feature は `api/src/features.rs` と `api/src/routers.rs` に明示的に登録する。
- 複数 feature で再利用する型やレスポンスは `api/src/shared/` に置く。単一 feature の都合だけで共通化しない。
- executable の初期化処理は `api/src/bin/server.rs` に限定し、ビジネスロジックを置かない。

## Axum と HTTP API

- handler は薄く保ち、次の順序を基本とする: extractor で入力取得、入力検証、command への変換、repository 呼び出し、model／response への変換。
- アプリケーション状態は `axum::extract::State<state::AppState>` で受け取る。新しい共有状態は `AppState` に追加し、安価に `Clone` できる型にする。
- JSON 入力は専用 request 型で受け、handler の先頭で `garde::Validate` により検証する。DB 制約だけを入力検証の代わりにしない。
- パス ID には `Uuid` を直接ばらまかず、`RegionId` や `WalkId` のような newtype を使う。
- 成功時の status code と header を明示する。作成は `201 Created` と `Location`、削除成功は `204 No Content` を基本とする。
- handler のエラー型は `errors::ApiError` に統一し、`IntoResponse` で status code と JSON エラー形式へ変換する。
- 内部エラーの詳細をクライアントに返さない。詳細は `tracing` へ記録し、レスポンスには安定した安全なメッセージを返す。
- `404 Not Found`、`400 Bad Request`、競合など、想定可能な失敗を適切な `ApiError` variant と status code で表す。
- route の追加・変更時は `utoipa::path` の method、path、params、request body、成功・失敗 response も同時に更新し、`api/src/openapi.rs` に登録する。
- middleware は原則として router/layer で適用し、各 handler 内で横断的処理を重複させない。

## 非同期処理

- 非同期 I/O には Tokio を使用し、async handler 内で blocking I/O や長時間の CPU 処理を直接実行しない。必要なら `tokio::task::spawn_blocking` を使用する。
- mutex guard、DB connection、transaction などのリソースを不要に `.await` をまたいで保持しない。
- detached task を安易に生成しない。生成する場合は失敗の観測方法と shutdown 時の扱いを明確にする。
- タイムアウト、キャンセル、graceful shutdown が必要な処理では `tokio-util` など既存の仕組みを優先する。

## SQLx と PostgreSQL

- SQL は読みやすさと再利用性のため `api/sql/` 配下の `.sql` ファイルに置くことを基本とし、短い診断クエリ以外は `query_file!` / `query_file_as!` を使う。
- 可能な限り SQLx のコンパイル時検査付き macro を使用する。型検査を避ける目的で動的 query に切り替えない。
- repository は `sqlx::Executor<Database = sqlx::Postgres>` を受け取り、pool connection と transaction の両方から呼べる形を維持する。
- 複数の書き込み、または一体として成功すべき処理は transaction にまとめる。transaction の開始・commit は原則 handler/service 側で行う。
- `SELECT *` を避け、取得列を明示する。入力値は必ず bind parameter とし、文字列連結で SQL を構築しない。
- DB row と公開 model を分離し、変換は `From` / `TryFrom` で明示する。DB 固有型を API 境界へ漏らさない。
- 共有環境で適用済み、または main branch で公開済みの migration は書き換えず、`db/migrations/` に修正用 migration を追加する。まだ共有・適用されていない開発中の migration は、適用状況を確認したうえで直接修正してよい。いずれの場合も、対応する query、row、model、seed、テストを必要に応じて更新する。
- SQLx macro の検査には接続可能な `DATABASE_URL` が必要である。秘密値をコミットせず、ローカル環境変数から渡す。

## エラー処理とログ

- `ApiError` には、利用者が取るべき対応や HTTP status が異なる場合に variant を追加する。
- DB の一意制約違反など、既知の DB エラーは可能なら意味のある API エラーへ変換する。
- ログには `tracing` を使用し、`println!` / `dbg!` を残さない。
- 5xx は `error!`、想定内の 4xx は必要に応じて `warn!` または `debug!` とし、通常の 404 や validation error を無条件にサーバ障害として扱わない。
- リクエスト本文、パスワード、token、完全な DB URL、個人情報をログへ記録しない。

## Rust の品質基準

- `cargo fmt` に従う。手作業で独自の整形を行わない。
- Clippy warning を新たに増やさない。lint を `allow` する場合は、局所的に適用して理由を記す。
- public item は必要最小限にする。所有権の都合だけで不必要な `clone()` を追加しない。
- 状態や ID、単位には primitive obsession を避け、newtype や既存の `uom` 型を使う。
- 網羅的に扱うべき enum には wildcard arm を使わず、variant 追加時にコンパイルエラーで気付けるようにする。
- import と module 宣言は rustfmt の出力に従い、未使用コードやデバッグコードを残さない。

## テスト

- 振る舞いを変更した場合は、その変更を失敗時も含めて検証するテストを追加・更新する。
- pure な変換、validation、error mapping は unit test、route、extractor、status/header/body は router に対する integration test で検証する。
- DB test はテスト間で状態を共有せず、transaction rollback または独立した fixture で再現可能にする。
- 最低限、正常系、validation error、対象なし、repository/DB failure のうち変更に関係するケースを確認する。
- テストで時刻、乱数、実行順序、外部ネットワークへ暗黙に依存しない。

## 変更後の確認

リポジトリルートで、変更範囲に応じて次を実行する。

```bash
cargo +nightly fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

- SQLx macro を含む確認では、migration 適用済み PostgreSQL と正しい `DATABASE_URL` を用意する。
- DB は必要な場合に `just dup` で起動する。コンテナの起動・停止や migration 実行で既存データへ影響し得る場合は、対象を確認してから行う。
- 環境不足で一部の確認を実行できない場合は、未実行のコマンドと理由を明記する。
- 完了時には変更内容、追加・変更した API、migration の有無、実行した確認と結果を簡潔に報告する。
