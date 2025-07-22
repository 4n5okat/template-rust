// Actix Webの主要コンポーネントをインポート
use actix_web::{App, HttpServer};

// ログ
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::EnvFilter;

// .envファイルの環境変数を読み込むためのクレート
use dotenvy::dotenv;

// ルーティングの初期化関数をインポート
use app::infrastructure::web::routes::init_routes;

// ログ設定の初期化
fn init_logger() {
    tracing_subscriber::fmt()
        .with_timer(UtcTime::rfc_3339()) // ISO 8601 timestamp
        .json()
        .with_span_events(FmtSpan::CLOSE) // span終了時に出力（任意）
        .with_target(true)
        .with_thread_ids(true)
        .with_env_filter(EnvFilter::from_default_env()) // RUST_LOG env変数に対応
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ロガーを初期化
    init_logger();

    // `.env` ファイルを読み込んで環境変数を設定
    dotenv().ok();

    // HTTPサーバーを起動
    HttpServer::new(move || {
        App::new()

            // ルーティング設定関数（init_routes）を適用
            .configure(init_routes)
    })
    .bind("0.0.0.0:8080")? // ポート8080でバインド
    .run()
    .await
}
