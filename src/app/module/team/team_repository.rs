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

            let collection = self.database.collection::<Team>("team");

            match collection.insert_one(dto, None).await {
                Ok(value) => {Ok(value)},
                Err(err) => {Err(AppError::DatabaseError(err.to_string()))}
            }

        }

        /*
        pub async fn get_user(&self, id: &String) -> Result<User, Error> {
            let obj_id = ObjectId::parse_str(id).unwrap();
            let filter = doc! {"_id": obj_id};
            let user_detail = self
                .col
                .find_one(filter, None)
                .await
                .ok()
                .expect("Error getting user's detail");
            Ok(user_detail.unwrap())
        }

        pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
            let obj_id = ObjectId::parse_str(id).unwrap();
            let filter = doc! {"_id": obj_id};
            let new_doc = doc! {
                "$set":
                    {
                        "id": new_user.id,
                        "name": new_user.name,
                        "location": new_user.location,
                        "title": new_user.title
                    },
            };
            let updated_doc = self
                .col
                .update_one(filter, new_doc, None)
                .await
                .ok()
                .expect("Error updating user");
            Ok(updated_doc)
        }

        pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
            let obj_id = ObjectId::parse_str(id).unwrap();
            let filter = doc! {"_id": obj_id};
            let user_detail = self
                .col
                .delete_one(filter, None)
                .await
                .ok()
                .expect("Error deleting user");
            Ok(user_detail)
        }

        pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
        }

         */
        

    }
}