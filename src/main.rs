use actix_web::{HttpServer,App};
use std::io::Result;
pub mod app;
use app::shared::database::connection::connection::init_pool;
use app::module::configure_all_modules;

#[actix_web::main]
async fn main() -> Result<()>{
    
    let port: u16 = 8080;
    let adress: &str = "127.0.0.1";

    // postgres://<username>:<password>@<host>:<port>/<database_name>
    let url_database = "postgres://postgres:FloriambraPostgres@localhost:5432/Matfilm";

    let db_pool = init_pool(url_database)
        .await
        .expect("Error connection database");

    let server = HttpServer::new(move || 
        App::new()
        .configure(|cfg| configure_all_modules(cfg, db_pool.clone()))
        .configure(app::module::configure_all_routers)
    ).bind((adress,port)).unwrap();

    println!("Server running port {port}");

    server.run().await

}
