pub(crate) mod process_handler {

    use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, ResponseError};
    use uuid::Uuid;
    use crate::app::{module::process::process_model::process_model::CreateUpdateProcess};

    use super::super::process_service::process_service::ProcessService;

    #[get("/process")]
    pub(crate) async fn get_all_process(service: web::Data<ProcessService>) -> impl Responder {
        match service.get_all().await {
            Ok(value) => {
                HttpResponse::Ok().json(value)
            },
            Err(err) => {   
                println!("Error get all process {err}");
                err.error_response()
            }
        }
    }

    #[get("/process/{id}")]
    pub(crate) async fn get_find_by_id_process(service: web::Data<ProcessService>,id: web::Path<Uuid>) -> impl Responder {
        match service.get_find_by_id(id.into_inner()).await {
            Ok(value) => {
                HttpResponse::Ok().json(value)
            },
            Err(err) => {
                println!("Error get find by id process {err}");
                err.error_response()
            }
        }
    }

    #[post("/process")]
    pub(crate) async fn create_process(service: web::Data<ProcessService>, dto: web::Json<CreateUpdateProcess>) -> impl Responder {
        
        match service.create(dto.into_inner()).await {
            Ok(value) => {
                HttpResponse::Created().json(value)
            },
            Err(err) => {
                println!("Error create new process {err}");
                err.error_response()
            }
        }
    }

    #[patch("/process/{id}")]
    pub(crate) async fn update_process(
        service: web::Data<ProcessService>,
        dto: web::Json<CreateUpdateProcess>,
        id: web::Path<Uuid>
    ) -> impl Responder {
    
        match service
            .update(dto.into_inner(), id.into_inner()).await {
            Ok(value) => {
                HttpResponse::Ok().json(value)
            },
            Err(err) => {
                println!("Error update process {err}");
                err.error_response()
            }
        }
    }

    #[delete("/process/{id}")]
    pub(crate) async fn delete_process(
        service: web::Data<ProcessService>,
        id: web::Path<Uuid>
    ) -> impl Responder {
        match service.delete(id.into_inner()).await {
            Ok(value) => {
                if let 1 = value {
                    HttpResponse::NoContent().finish()
                } else {
                    HttpResponse::NotFound().finish()
                }
                
            },
            Err(err) => {
                println!("Error delete process {err}");
                err.error_response()
            }
        }
    }

}