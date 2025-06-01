pub mod connection_mongo {
    use mongodb::{error::Error, options::ClientOptions, Client, Database};

    

    pub async fn init_conection(databse_url:String,client_db: String) -> Result<Database,Error> {
        let client_options = ClientOptions::parse(&databse_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(&client_db);
        Ok(db)
    } 

}