pub(crate) mod process_router {
    use actix_web::web::ServiceConfig;
    
    use super::super::process_handler::process_handler::{
        get_all_process,
        get_find_by_id_process,
        create_process,
        update_process,
        delete_process
    }
        ;

    pub(crate) fn configure_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(get_all_process)
        .service(get_find_by_id_process)
        .service(create_process)
        .service(update_process)
        .service(delete_process);

    }
}