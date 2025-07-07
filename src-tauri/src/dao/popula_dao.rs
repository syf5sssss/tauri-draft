use crate::commands::sqlx::POOL;
use crate::dto::Popula;
use anyhow::{ Context, Result };
use sqlx::MySqlPool;

pub struct PopulaDao;

impl PopulaDao {
    /// 分页获取书籍列表
    pub async fn list() -> Result<Vec<Popula>> {
        let pool = Self::get_pool().await?;
        // 获取数据
        let populas = sqlx
            ::query_as::<_, Popula>(r#"
            SELECT * 
            FROM yearbook_popula
            "#)
            .fetch_all(&pool).await
            .context("Failed to list populas")?;

        Ok(populas)
    }

    /// 获取数据库连接池
    async fn get_pool() -> Result<MySqlPool> {
        let guard = POOL.lock().await;
        let pool = guard.as_ref().context("Database connection not initialized")?.clone();
        Ok(pool)
    }
}
