pub mod team_repository {
    use futures::TryStreamExt;
    use mongodb::{bson::{doc, oid::ObjectId}, results::InsertOneResult, Database};
    use super::super::team_model::Team;
    use crate::app::shared::error::error_app::error_app::AppError;

    #[derive(Clone)]
    pub(crate) struct TeamRepository {
        database: Database
    }

    impl TeamRepository {
        pub fn new(database: Database) -> Self {
            Self { database }
        }

        pub async fn create(&self,dto: Team) -> Result<InsertOneResult,AppError> {

            let collection = self.database.collection::<Team>("team");

            match collection.insert_one(dto, None).await {
                Ok(value) => {Ok(value)},
                Err(err) => {Err(AppError::DatabaseError(err.to_string()))}
            }

        }

        pub async fn find_all(&self) -> Result<Vec<Team>,AppError> {
            let collection = self.database.collection::<Team>("team");
            let mut list_team_result: Vec<Team> = vec![];

            match collection.find(None, None).await {
                Ok(mut value) => {
                    while let Ok(Some(result)) = value.try_next().await.map(|value| value) {
                        list_team_result.push(result);
                    } 
                    Ok(list_team_result)
                },
                Err(err) => {Err(AppError::DatabaseError(err.to_string()))}
            }
        }

        pub async fn find_name(&self,param: String) -> Result<Vec<Team>,AppError> {
            let collection = self.database.collection::<Team>("team");
            let mut list_team_result: Vec<Team> = vec![];

            match collection.find(doc!{"name": param}, None).await {
                Ok(mut value) => {
                    while let Ok(Some(result)) = value.try_next().await.map(|value| value) {
                        list_team_result.push(result);
                    } 
                    Ok(list_team_result)
                },
                Err(err) => {Err(AppError::DatabaseError(err.to_string()))}
            }
        }

        pub async fn update(&self,dto: Team,param: String) -> Result<u64,AppError> {
            let collection = self.database.collection::<Team>("team");
            let obj_id = ObjectId::parse_str(param).unwrap();
            let filter = doc!{ "_id":obj_id };
            let price: u32 = dto.price as u32;
            let update = doc! {"$set": doc! {"price": price, "name": dto.name } };

            match collection.update_one(filter, update, None).await {
                Ok(value) => {
                    Ok(value.matched_count)
                },
                Err(err) => {
                    Err(AppError::DatabaseError(err.to_string()))
                }
            }
        }

        pub async fn delete(&self,param: String) -> Result<u64,AppError> {
            let collection = self.database.collection::<Team>("team");
            let obj_id = ObjectId::parse_str(param).unwrap();
            let filter = doc! {"$and": [ doc! {"_id": obj_id } ]};

            match collection.delete_one(filter, None).await {
                Ok(value) => {
                    Ok(value.deleted_count)
                },
                Err(err) => {
                    Err(AppError::DatabaseError(err.to_string()))
                }
            }
        }
        
    }
}