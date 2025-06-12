use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub total: i32,
    pub current_page: i32,
    pub page_size: i32,
    pub page_count: i32,
}
