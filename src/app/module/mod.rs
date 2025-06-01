use actix_web::web;
use mongodb::Database;
use sqlx::PgPool;

pub mod team;
pub mod process;


pub fn configure_all_modules(cfg: &mut web::ServiceConfig,db_pool:PgPool,db_mongo: Database) {
    
    cfg
        .configure(move |cfg |process::configure_services(cfg, db_pool))
        .configure(|cfg|team::configure_services(cfg, db_mongo));
}

pub fn configure_all_routers(cfg: &mut web::ServiceConfig) {
    cfg.configure(team::configure_routes)
    .configure(process::configure_routes);
}