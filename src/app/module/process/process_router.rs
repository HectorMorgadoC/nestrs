pub(crate) mod process_router {
    use actix_web::web::{ServiceConfig,get,patch,post,delete};
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
        .route("process", get().to(get_all_process))
        .route("process", post().to(create_process))
        .route("process/{id}", get().to(get_find_by_id_process))
        .route("process/{id}", patch().to(update_process))
        .route("process/{id}", delete().to(delete_process));
    }
}