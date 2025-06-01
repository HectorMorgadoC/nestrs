pub mod team_repository {
    use mongodb::{results::InsertOneResult, Client, Collection, Database};
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

        pub async fn create_team(&self,dto: Team) -> Result<InsertOneResult,AppError> {
            //let collection = self.database.insert_one(dto, None).await;

            let collection = self.database.collection::<Team>("team");

            match collection.insert_one(dto, None).await {
                Ok(value) => {Ok(value)},
                Err(err) => {Err(AppError::DatabaseError(err.to_string()))}
            }

        }
    }
}