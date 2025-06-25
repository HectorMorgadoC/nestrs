use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use super::{super::team::team_model::Team, team_service::team_service::TeamService};

#[post("/team")]
pub async fn create_team(service: web::Data<TeamService>, dto: web::Json<Team>) -> impl Responder {
    match service.create(dto.into_inner()).await {
        Ok(value) => {HttpResponse::Ok().json(value)},
        Err(err) => {
            println!("Err created team: {}",err);
            HttpResponse::Ok().body("Err al crear")
        }
    }
}

#[get("/team")]
pub async fn get_find_all_teams(service: web::Data<TeamService>) -> impl Responder {
    match service.find_all().await {
        Ok(value) => {
            HttpResponse::Ok().json(value)
        },
        Err(err) => {
            println!("Err find all team: {}",err);
            HttpResponse::Ok().body("Err pedit los equipos")
        }
    }
}

#[get("/team/{name}")]
pub async fn get_find_name_teams(service: web::Data<TeamService>, param: web::Path<String>) -> impl Responder {
    match service.find_name(param.into_inner()).await {
        Ok(value) => {
            HttpResponse::Ok().json(value)
        },
        Err(err) => {
            println!("Err find all team: {}",err);
            HttpResponse::Ok().body("Err pedir los equipos por su nombre")
        }
    }

}

#[patch("/team/{id}")]
pub async fn update_team(
    service: web::Data<TeamService>,
    dto: web::Json<Team>,
    param: web::Path<String>
) -> impl Responder {
    match  service.update(dto.into_inner(), param.into_inner()).await {
        Ok(value) => {
            HttpResponse::Ok().body(value)
        },
        Err(err) => {
            HttpResponse::Ok().body(format!("{:?}",err))
        }
    }
}

#[delete("/team/{id}")]
pub async fn delete_team(
    service: web::Data<TeamService>,
    param: web::Path<String>
) -> impl Responder {
    match  service.delete(param.into_inner()).await {
        Ok(value) => {
            HttpResponse::Ok().body(value)
        },
        Err(err) => {
            HttpResponse::Ok().body(format!("{:?}",err))
        }
    }
}