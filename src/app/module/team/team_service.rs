pub mod team_service {
    use actix_web::cookie::time::format_description::parse;
    use mongodb::bson::oid::ObjectId;

    use crate::app::module::team;
    use crate::app::shared::error::error_app::error_app::AppError;

    use super::super::team_repository::team_repository::TeamRepository;
    use super::super::team_model::Team;

    pub(crate) struct TeamService {
        repository: TeamRepository,
    }

    impl TeamService {
        pub(crate) fn new(repository: TeamRepository) -> Self {
            Self { repository }
        }

        pub async fn create(&self,dto: Team) -> Result<Team,AppError> {
            let new_team = self.repository.create_team(dto.clone()).await;
            match new_team {
                Ok(value) => {
                    let team_result: Team = Team { 
                        id: value.inserted_id.as_object_id(), 
                        name: dto.name, 
                        price: dto.price
                    };
                    Ok(team_result)
                },
                Err(err) => Err(err)
            }
        }
    }

}