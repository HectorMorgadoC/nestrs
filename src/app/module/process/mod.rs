
use actix_web::web;
use sqlx::PgPool;

mod process_model;
mod process_repository;
mod process_service;
mod process_router;
mod process_handler;

use process_repository::process_repository::ProcessRepository;



pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(process_router::process_router::configure_routes);
}

pub fn configure_services(cfg: &mut web::ServiceConfig, db_pool: PgPool) {
    let process_repository: ProcessRepository = process_repository::process_repository::ProcessRepository::new(db_pool.clone());
    let process_service = process_service::process_service::ProcessService::new(process_repository);

    cfg.app_data(web::Data::new(process_service));
}
