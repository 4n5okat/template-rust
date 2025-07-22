// Actix Webの主要コンポーネントをインポート
use actix_web::{get, HttpResponse, Responder};

use tracing::{info};

#[get("/")]
async fn hello() -> impl Responder {
    info!("Access");
    HttpResponse::Ok().body("Hello, world!")
}
