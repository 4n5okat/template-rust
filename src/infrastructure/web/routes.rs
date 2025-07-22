// Actix Web の web モジュールをインポート（ルーティング設定に使用）
use actix_web::web;

//
use crate::interface::controllers::sample::hello;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // アクセストークン発行
    cfg.service(hello);
}
