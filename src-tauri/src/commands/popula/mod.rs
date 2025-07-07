use crate::dao::PopulaDao;
use crate::dto::Popula;
use tauri::command;

#[command]
pub async fn popula_list() -> Result<Vec<Popula>, String> {
    match PopulaDao::list().await {
        Ok(populas) => Ok(populas),
        Err(e) => {
            let error_message = format!("Failed to list populas: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
}
