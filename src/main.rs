use actix_web::{HttpServer,App};
use std::env;
use std::io::Result;
pub mod app;
use crate::app::shared::database::{connection::connection::init_pool,connection_mongo::connection_mongo::init_conection};
use crate::app::module::configure_all_modules;

#[actix_web::main]
async fn main() -> Result<()>{
    dotenv::dotenv().ok();
    let port: u16 = 8081;
    let adress: &str = "127.0.0.1";

    //postgres://<username>:<password>@<host>:<port>/<database_name>
    let url_database = "postgres://postgres:FloriambraPostgres@localhost:5432/Matfilm";

    let db_pool = init_pool(url_database)
        .await
        .expect("Error connection database");

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let db_mongo = init_conection(mongo_uri, "mydb".to_string()).await.unwrap();

    let server = HttpServer::new(move || 
        App::new()
        .configure(|cfg| configure_all_modules(cfg, db_pool.clone(),db_mongo.clone()))
        .configure(app::module::configure_all_routers)
    ).bind((adress,port)).unwrap();

    println!("Server running port {port}");

    server.run().await

}
