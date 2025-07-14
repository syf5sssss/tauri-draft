use chrono::{DateTime, NaiveDateTime};
use serde::{de, Deserialize};

pub fn parse_publish_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
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
