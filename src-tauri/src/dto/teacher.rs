use chrono::NaiveDateTime;
use crate::dto::parse::parse_publish_date;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub height: f64,
    #[serde(deserialize_with = "parse_publish_date")]
    pub birthday: NaiveDateTime,
}

impl Teacher {
    pub fn new(id: i32, name: String, age: i32, height: f64, birthday: NaiveDateTime) -> Self {
        Teacher {
            id,
            name,
            age,
            height,
            birthday,
        }
    }
}
