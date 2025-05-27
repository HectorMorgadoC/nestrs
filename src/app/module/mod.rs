use actix_web::web;
use sqlx::PgPool;

pub mod team;
pub mod process;

pub fn configure_all_modules(cfg: &mut web::ServiceConfig,db_pool:PgPool) {
    cfg
        .configure(move |cfg |process::configure_services(cfg, db_pool));
}

pub fn configure_all_routers(cfg: &mut web::ServiceConfig) {
    cfg.configure(process::configure_routes);
}