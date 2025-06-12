use chrono::{DateTime, NaiveDateTime};
use serde::{de, Deserialize, Serialize};
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

fn parse_publish_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // 尝试解析 ISO 格式 (如 "2025-06-27T17:21:28Z")
    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
        // 转换为本地时区，然后提取 NaiveDateTime
        return Ok(dt.with_timezone(&chrono::Local).naive_local());
    }

    // 尝试解析 JavaScript 的日期字符串 (如 "Fri Jun 27 2025 17:21:28 GMT+0800")
    if let Ok(dt) = DateTime::parse_from_str(&s, "%a %b %d %Y %H:%M:%S GMT%z") {
        // 转换为本地时区，然后提取 NaiveDateTime
        return Ok(dt.with_timezone(&chrono::Local).naive_local());
    }

    // 尝试自定义格式
    if let Ok(ndt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
        return Ok(ndt);
    }

    Err(de::Error::custom(format!("无法解析日期: {}", s)))
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
