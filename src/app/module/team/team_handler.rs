use actix_web::{post, web, HttpResponse, Responder};
use mongodb::{bson::doc, Database};
use super::{super::team::team_model::Team, team_service::team_service::TeamService};
use futures::TryStreamExt;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teams")
            //.route("", web::post().to(create_team))
            .route("", web::get().to(get_teams))
    );
}


/* 
async fn create_team(db: web::Data<Database>, team: web::Json<Team>) -> impl Responder {
    let collection = db.collection::<Team>("teams");
    let mut new_team = team.into_inner();
    new_team.id = None;

    match collection.insert_one(new_team, None).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Error inserting team")
        }
    }
}
*/

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

async fn get_teams(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<Team>("teams");

    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut teams = Vec::new();
            while let Some(team) = cursor.try_next().await.unwrap_or(None) {
                teams.push(team);
            }
            HttpResponse::Ok().json(teams)
        }
        Err(e) => {
            println!("Find error: {:?}", e);
            HttpResponse::InternalServerError().body("Error fetching teams")
        }
    }
}
