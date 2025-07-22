# =============================================================================
# Builder Stage 1: cargo-chef
# 依存関係のみをビルドし、キャッシュを効率化するためのステージ
# =============================================================================
FROM rust:1.88-slim-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
# 依存関係のビルドプランを生成
RUN cargo chef prepare --recipe-path recipe.json

# =============================================================================
# Builder Stage 2: Caching Dependencies
# 生成されたプランを元に依存関係をコンパイルする
# =============================================================================
FROM rust:1.88-slim-bookworm AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=chef /app/recipe.json recipe.json
# 依存関係のみをビルドしてキャッシュレイヤーを作成
RUN cargo chef cook --release --recipe-path recipe.json

# =============================================================================
# Builder Stage 3: Application Builder
# アプリケーションコードをビルドする
# =============================================================================
FROM rust:1.88-slim-bookworm AS builder
WORKDIR /app
# キャッシュ済みの依存関係をコピー
COPY --from=planner /app/target /app/target
COPY --from=planner /usr/local/cargo /usr/local/cargo
COPY . .
# アプリケーションコードをビルド
RUN cargo build --release

# =============================================================================
# Development Stage
# 開発用の環境。ホットリロードツール(cargo-watch)を含む
# =============================================================================
FROM builder AS development
# ホットリロード用にcargo-watchをインストール
RUN cargo install cargo-watch

# ログレベルの設定
ENV RUST_LOG=info
# 開発サーバーの起動コマンド
CMD ["cargo", "watch", "-q", "-c", "-w", "src/", "-x", "run"]

# =============================================================================
# Production Stage
# 実行に必要な最小限のファイルのみを含む本番用イメージ
# =============================================================================
FROM debian:bookworm-slim AS production
WORKDIR /app
# ビルドされたバイナリのみをbuilderステージからコピー
COPY --from=builder /app/target/release/app .
# ポートを開放
EXPOSE 8080

# ログレベルの設定
ENV RUST_LOG=info

# コンテナ起動時のコマンド
CMD ["./app"]
