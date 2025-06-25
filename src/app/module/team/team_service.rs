pub mod team_service {
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
            let new_team = self.repository.create(dto.clone()).await;
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

        pub async fn find_all(&self) -> Result<Vec<Team>,AppError> {
            let list_teams = self.repository.find_all().await;
            match list_teams {
                Ok(value) =>  {
                    Ok(value)
                },
                Err(err) => {
                    Err(err)
                }
                    
            }
        }

        pub async fn find_name(&self,param: String) -> Result<Vec<Team>,AppError> {
            let list_teams = self.repository.find_name(param).await;
            match list_teams {
                Ok(value) =>  {
                    Ok(value)
                },
                Err(err) => {
                    Err(err)
                }
            }
        }

        pub async fn update(&self,dto: Team,param: String) -> Result<String,AppError> {
            let matchet_count = self.repository.update(dto, param).await;
            match matchet_count {
                Ok(value) => {
                    if value != 1 {
                        return  Ok(String::from("No se actualizo registro"))
                    } 
                    Ok(String::from("Registro actualizado"))
                },
                Err(err) => {
                    Err(err)
                }
            }
        }

        pub async fn delete(&self,param: String) -> Result<String,AppError> {
            let delete_count = self.repository.delete(param).await;
            match delete_count {
                Ok(value) => {
                    if value != 1 {
                        return  Ok(String::from("No se elimino el registro"))
                    } 
                    Ok(String::from("Registro eliminado"))
                },
                Err(err) => {
                    Err(err)
                }
            }
        }

    }

}