use sqlx::FromRow;
use chrono::{ DateTime, Utc };
use serde::{ Serialize, Deserialize };

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: i32,
    pub price: f64,
    pub sales: i64,
    pub publish_date: DateTime<Utc>,
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
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub rating: Option<i32>,
    pub min_rating: Option<i32>,
    pub max_rating: Option<i32>,
    pub min_sales: Option<i64>,
    pub max_sales: Option<i64>,
    pub min_publish_date: Option<DateTime<Utc>>,
    pub max_publish_date: Option<DateTime<Utc>>,
    pub img: Option<String>,
    pub status: Option<String>,
}
