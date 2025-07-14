use crate::dto::parse::parse_publish_date;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub height: f64,
    #[serde(deserialize_with = "parse_publish_date")]
    pub birthday: NaiveDateTime,
}

impl Student {
    pub fn new(id: i32, name: String, age: i32, height: f64, birthday: NaiveDateTime) -> Self {
        Student {
            id,
            name,
            age,
            height,
            birthday,
        }
    }
}
