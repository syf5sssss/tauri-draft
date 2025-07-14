use crate::dto::parse::parse_publish_date;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub price: f64,
    pub sales: i64,
    #[serde(deserialize_with = "parse_publish_date")]
    pub publish_date: NaiveDateTime,
    pub title: String,
    pub author: String,
    pub category: String,
    pub rating: i32,
    pub img: String,
    pub status: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BookQuery {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub price: Option<f64>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub rating: Option<i32>,
    pub min_rating: Option<i32>,
    pub max_rating: Option<i32>,
    pub sales: Option<i64>,
    pub min_sales: Option<i64>,
    pub max_sales: Option<i64>,
    pub publish_date: Option<NaiveDateTime>,
    pub min_publish_date: Option<NaiveDateTime>,
    pub max_publish_date: Option<NaiveDateTime>,
    pub img: Option<String>,
    pub status: Option<String>,
}
