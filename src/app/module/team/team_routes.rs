pub mod team_router {
    use actix_web::web::ServiceConfig;
    use super::super::team_handler::*;

    pub fn configure_routes(cfg: &mut ServiceConfig) {
        cfg.service(create_team)
        .service(get_find_all_teams)
        .service(get_find_name_teams)
        .service(update_team)
        .service(delete_team);
    }
}