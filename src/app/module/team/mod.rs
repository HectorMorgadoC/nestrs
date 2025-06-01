use actix_web::web;
use team_model::Team;
use mongodb::{Collection, Database};
use team_repository::team_repository::TeamRepository;

mod team_model;
pub mod team_handler;
mod team_repository;
mod team_service;
mod team_routes;


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(team_routes::team_router::configure_routes);
}

pub fn configure_services(cfg: &mut web::ServiceConfig, db: Database) {
    let team_repository: TeamRepository = team_repository::team_repository::TeamRepository::new(db.clone());
    let team_service = team_service::team_service::TeamService::new(team_repository);

    cfg.app_data(web::Data::new(team_service));
}