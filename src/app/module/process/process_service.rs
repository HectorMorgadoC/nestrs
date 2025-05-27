pub mod process_service {

    use uuid::Uuid;

    use crate::app::module::process::process_model::process_model::CreateUpdateProcess;
    use crate::app::shared::error::error_app::error_app::AppError;

    use super::super::process_repository::process_repository::ProcessRepository;
    use super::super::process_model::process_model::Process;
    pub(crate) struct ProcessService {
        repository: ProcessRepository,
    }

    impl ProcessService {
        pub fn new(repository: ProcessRepository) -> Self {
            Self { repository }
        }

        pub async fn get_all(&self) -> Result<Vec<Process>,AppError> {
            let process = self
                .repository
                .get_all_processes()
                .await;
            process
        }

        pub async fn get_find_by_id(&self,id: Uuid) -> Result<Process,AppError> {
            let process = self
                .repository
                .get_process_by_id(id)
                .await;
            process
        }

        pub async fn create(&self,dto: CreateUpdateProcess) -> Result<Process,AppError> {
            let new_process = self
                .repository
                .create_process(dto)
                .await;
            new_process
        }

        pub async fn update(&self,dto: CreateUpdateProcess, id: Uuid) -> Result<Process,AppError> {
            let modified_process = self
                .repository
                .update_process(id, dto)
                .await;
            modified_process
        }

        pub async fn delete(&self,id: Uuid) -> Result<u64,AppError> {
            let delete_process = self
            .repository
            .delete_process(id)
            .await;
            delete_process
        }
    }

    
}