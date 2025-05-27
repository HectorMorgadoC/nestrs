pub(crate) mod process_model {

    use serde::{Deserialize, Serialize};
    use validator::Validate;
    use uuid::Uuid;
    #[derive(sqlx::FromRow,Clone,Debug,Default,Validate,Serialize)]
    pub(crate) struct Process {
        pub id: Uuid,
        pub name: String,
        pub description: String,
        pub is_active: bool
    }

    #[derive(Deserialize)]
    pub(crate) struct CreateUpdateProcess {
        pub name: String,
        pub description: String,
        pub is_active: bool
    }

}