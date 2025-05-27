pub mod process_repository {

    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::app::shared::error::error_app::error_app::AppError;
    use super::super::process_model::process_model::{ CreateUpdateProcess, Process };

    pub(crate) struct ProcessRepository {
        pool: PgPool
    }

    impl ProcessRepository {
        
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }

        pub async fn create_process(&self, dto: CreateUpdateProcess) -> Result<Process, AppError> {
            let row = sqlx::query_as::<_, Process>(
                "INSERT INTO process (name, description, is_active)
                    VALUES ($1, $2, $3)
                    RETURNING id, name, description, is_active"
            )
            .bind(&dto.name)
            .bind(&dto.description)
            .bind(dto.is_active)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string())).unwrap();
        
            Ok(row)
        }

        pub async fn get_all_processes(&self) -> Result<Vec<Process>, AppError> {
            let rows = sqlx::query_as::<_, Process>("SELECT id, name, description, is_active FROM process")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string())).unwrap();
        
            Ok(rows)
        }

        pub async fn get_process_by_id(&self, id: Uuid) -> Result<Process, AppError> {
            let row = sqlx::query_as::<_, Process>(
                "SELECT id, name, description, is_active FROM process WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string())).unwrap();
        
            Ok(row)
        }

        pub async fn update_process(&self, id: Uuid, dto: CreateUpdateProcess) -> Result<Process, AppError> {
            let row = sqlx::query_as::<_, Process>(
                "UPDATE process
                    SET name = $1, description = $2, is_active = $3
                    WHERE id = $4
                    RETURNING id, name, description, is_active"
            )
            .bind(&dto.name)
            .bind(&dto.description)
            .bind(dto.is_active)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string())).unwrap();
        
            Ok(row)
        }

        pub async fn delete_process(&self, id: Uuid) -> Result<u64, AppError> {
            let result = sqlx::query("DELETE FROM process WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string())).unwrap();
        
            Ok(result.rows_affected())
        }

    }

}