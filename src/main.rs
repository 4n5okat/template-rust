// Actix Webの主要コンポーネントをインポート
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// ログ
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::EnvFilter;

use tracing::{info, warn, error, span, Level};

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

#[get("/")]
async fn hello() -> impl Responder {
    info!("Access");
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ロガーを初期化
    init_logger();

    // HTTPサーバーを起動
    HttpServer::new(move || {
        App::new()
            .service(hello) // ハンドラーをサービスに登録
    })
    .bind("0.0.0.0:8080")? // ポート8080でバインド
    .run()
    .await
}
