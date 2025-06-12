use chrono::{DateTime, Utc};
use sqlx::{Database, Encode, QueryBuilder, Type};

// 参数类型枚举
#[derive(Clone)]
pub enum Param {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    DateTime(DateTime<Utc>),
    Boolean(bool),
}

impl Param {
    pub fn bind<'a, DB>(&self, builder: &mut QueryBuilder<'a, DB>)
    where
        DB: Database,
        i32: Encode<'a, DB> + Type<DB>,
        i64: Encode<'a, DB> + Type<DB>,
        f32: Encode<'a, DB> + Type<DB>,
        f64: Encode<'a, DB> + Type<DB>,
        String: Encode<'a, DB> + Type<DB>,
        &'a str: Encode<'a, DB> + Type<DB>,
        bool: Encode<'a, DB> + Type<DB>,
        DateTime<Utc>: Encode<'a, DB> + Type<DB>,
    {
        // 处理字符串类型
        if let Param::String(v) = self {
            let sql = builder.sql();
            let is_like = sql.contains(" LIKE ?")
                || sql.ends_with(" LIKE ?")
                || sql.contains(" LIKE ? ")
                || sql.contains(" LIKE ?,");

            if is_like && !v.contains('%') {
                builder.push_bind(format!("%{}%", v));
            } else {
                builder.push_bind(v.clone());
            }
            return;
        }

        // 处理其他类型 - 使用值而不是引用
        match self {
            // 对于实现了 Copy 的类型，使用值
            Param::I32(v) => {
                builder.push_bind(*v);
            }
            Param::I64(v) => {
                builder.push_bind(*v);
            }
            Param::F32(v) => {
                builder.push_bind(*v);
            }
            Param::F64(v) => {
                builder.push_bind(*v);
            }
            Param::Boolean(v) => {
                builder.push_bind(*v);
            } // 解引用 bool 值

            // 对于非 Copy 类型，使用 clone()
            Param::DateTime(v) => {
                builder.push_bind(v.clone());
            }

            Param::String(_) => unreachable!(),
        }
    }
}

// 实现From trait以便转换
impl From<i32> for Param {
    fn from(i: i32) -> Self {
        Param::I32(i)
    }
}

impl From<i64> for Param {
    fn from(i: i64) -> Self {
        Param::I64(i)
    }
}

impl From<f32> for Param {
    fn from(f: f32) -> Self {
        Param::F32(f)
    }
}

impl From<f64> for Param {
    fn from(f: f64) -> Self {
        Param::F64(f)
    }
}

impl From<String> for Param {
    fn from(s: String) -> Self {
        Param::String(s)
    }
}

impl From<&str> for Param {
    fn from(s: &str) -> Self {
        Param::String(s.to_string())
    }
}

impl From<&String> for Param {
    fn from(s: &String) -> Self {
        Param::String(s.clone())
    }
}

impl From<DateTime<Utc>> for Param {
    fn from(dt: DateTime<Utc>) -> Self {
        Param::DateTime(dt)
    }
}

impl From<bool> for Param {
    fn from(b: bool) -> Self {
        Param::Boolean(b)
    }
}
