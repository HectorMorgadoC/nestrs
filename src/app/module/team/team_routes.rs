pub mod team_router {
    use actix_web::web::ServiceConfig;
    use super::super::team_handler::create_team;

    pub fn configure_routes(cfg: &mut ServiceConfig) {
        cfg.service(create_team);
    }
}