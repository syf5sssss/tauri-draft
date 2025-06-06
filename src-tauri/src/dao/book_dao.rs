use crate::{ commands::sqlx::POOL, dto::{ book::Book, BookQuery, Page } };
use chrono::Utc;
use sqlx::{ FromRow, MySqlPool, Row };
use anyhow::{ Result, Context };
use sqlx::{ QueryBuilder, MySql };

/// Book 表的数据访问对象
pub struct BookDao;

impl BookDao {
    /// 创建新书籍
    pub async fn create(book: &Book) -> Result<u64> {
        let pool = Self::get_pool().await?;

        let result = sqlx
            ::query(r#"
            INSERT INTO book (
                price, sales, publish_date, title, author, category, rating, img, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(book.price)
            .bind(book.sales)
            .bind(book.publish_date)
            .bind(&book.title)
            .bind(&book.author)
            .bind(&book.category)
            .bind(&book.rating)
            .bind(&book.img)
            .bind(&book.status)
            .execute(&pool).await
            .context("Failed to create book")?;

        Ok(result.last_insert_id())
    }

    /// 根据 ID 获取书籍
    pub async fn get_by_id(id: u32) -> Result<Option<Book>> {
        let pool = Self::get_pool().await?;

        let book = sqlx
            ::query_as::<_, Book>(r#"
            SELECT id, price, sales, publish_date, title, author, category, rating, img, status
            FROM book
            WHERE id = ?
            "#)
            .bind(id)
            .fetch_optional(&pool).await
            .context("Failed to get book by ID")?;

        Ok(book)
    }

    /// 更新书籍信息
    pub async fn update(book: &Book) -> Result<bool> {
        let pool = Self::get_pool().await?;

        let rows_affected = sqlx
            ::query(
                r#"
            UPDATE book SET
                price = ?,
                sales = ?,
                publish_date = ?,
                title = ?,
                author = ?,
                category = ?,
                rating = ?,
                img = ?
                status = ?
            WHERE id = ?
            "#
            )
            .bind(book.price)
            .bind(book.sales)
            .bind(book.publish_date)
            .bind(&book.title)
            .bind(&book.author)
            .bind(&book.category)
            .bind(&book.rating)
            .bind(&book.img)
            .bind(&book.status)
            .bind(book.id)
            .execute(&pool).await
            .context("Failed to update book")?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    /// 根据 ID 删除书籍
    pub async fn delete(id: u32) -> Result<bool> {
        let pool = Self::get_pool().await?;

        let rows_affected = sqlx
            ::query(r#"
            DELETE FROM book
            WHERE id = ?
            "#)
            .bind(id)
            .execute(&pool).await
            .context("Failed to delete book")?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    /// 分页获取书籍列表
    pub async fn list(page: u32, page_size: u32) -> Result<Vec<Book>> {
        let pool = Self::get_pool().await?;
        let offset = (page - 1) * page_size;
        println!("{:?}-{:?}", offset, page_size);
        // 获取数据
        let books = sqlx
            ::query_as::<_, Book>(r#"
            SELECT id, price, sales, publish_date, title, author, category, rating, img, status
            FROM book
            ORDER BY publish_date DESC
            LIMIT ? OFFSET ?
            "#)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&pool).await
            .context("Failed to list books")?;

        Ok(books)
    }

    pub async fn search(query: &str, page: i32, page_size: i32) -> Result<Page<Book>> {
        let pool = Self::get_pool().await?;
        let offset = (page - 1) * page_size;
        let search_pattern = format!("%{}%", query);

        let str =
            r#"SELECT id, price, sales, publish_date, title, author, category, rating, img, status
            FROM book
            WHERE title LIKE ? OR author LIKE ? OR category LIKE ?
            ORDER BY publish_date DESC
            LIMIT ? OFFSET ?"#;
        // 获取数据
        let books = sqlx::query_as::<_, Book>(&str).bind(&search_pattern).bind(&search_pattern).bind(&search_pattern).bind(page_size).bind(offset).fetch_all(&mut *pool.acquire().await?).await.context("Failed to search books")?;

        // 获取总数
        let total: i32 = sqlx
            ::query_scalar(r#"
            SELECT COUNT(*) 
            FROM book
            WHERE title LIKE ? OR author LIKE ? OR category LIKE ?
            "#)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .fetch_one(&mut *pool.acquire().await?).await
            .context("Failed to get search count")?;

        let pres = Page {
            data: books,
            page_count: ((total as f32) / (page_size as f32)).ceil() as i32,
            current_page: page,
            total,
            page_size,
        };
        Ok(pres)
    }

    pub async fn dynamics_search(query: &BookQuery, page: i32, page_size: i32) -> Result<Page<Book>> {
        println!("{:?}", query);
        let pool = Self::get_pool().await?;
        let offset = (page - 1) * page_size;

        let mut data_builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM book WHERE 1=1 ");
        let mut count_builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT COUNT(*) FROM book WHERE 1=1 ");

        if let Some(id) = query.id {
            data_builder.push(" AND id = ");
            data_builder.push_bind(format!("{}", id));
            count_builder.push(" AND id = ");
            count_builder.push_bind(format!("{}", id));
        }
        if let Some(title) = &query.title {
            data_builder.push(" AND title like ");
            data_builder.push_bind(format!("%{}%", title));
            count_builder.push(" AND title like ");
            count_builder.push_bind(format!("%{}%", title));
        }
        if let Some(author) = &query.author {
            data_builder.push(" AND author like ");
            data_builder.push_bind(format!("%{}%", author));
            count_builder.push(" AND author like ");
            count_builder.push_bind(format!("%{}%", author));
        }
        if let Some(img) = &query.img {
            data_builder.push(" AND img like ");
            data_builder.push_bind(format!("%{}%", img));
            count_builder.push(" AND img like ");
            count_builder.push_bind(format!("%{}%", img));
        }
        if let Some(category) = &query.category {
            data_builder.push(" AND category = ");
            data_builder.push_bind(format!("{}", category));
            count_builder.push(" AND category = ");
            count_builder.push_bind(format!("{}", category));
        }
        if let Some(status) = &query.status {
            data_builder.push(" AND status = ");
            data_builder.push_bind(format!("{}", status));
            count_builder.push(" AND status = ");
            count_builder.push_bind(format!("{}", status));
        }
        if let Some(rating) = query.rating {
            data_builder.push(" AND rating = ");
            data_builder.push_bind(format!("{}", rating));
            count_builder.push(" AND rating = ");
            count_builder.push_bind(format!("{}", rating));
        }
        if let Some(min_rating) = query.min_rating {
            data_builder.push(" AND rating >= ");
            data_builder.push_bind(format!("{}", min_rating));
            count_builder.push(" AND rating >= ");
            count_builder.push_bind(format!("{}", min_rating));
        }
        if let Some(max_rating) = query.max_rating {
            data_builder.push(" AND rating <= ");
            data_builder.push_bind(format!("{}", max_rating));
            count_builder.push(" AND rating <= ");
            count_builder.push_bind(format!("{}", max_rating));
        }
        if let Some(min_price) = query.min_price {
            data_builder.push(" AND price >= ");
            data_builder.push_bind(format!("{}", min_price));
            count_builder.push(" AND price >= ");
            count_builder.push_bind(format!("{}", min_price));
        }
        if let Some(max_price) = query.max_price {
            data_builder.push(" AND price <= ");
            data_builder.push_bind(format!("{}", max_price));
            count_builder.push(" AND price <= ");
            count_builder.push_bind(format!("{}", max_price));
        }
        if let Some(min_sales) = query.min_sales {
            data_builder.push(" AND sales >= ");
            data_builder.push_bind(format!("{}", min_sales));
            count_builder.push(" AND sales >= ");
            count_builder.push_bind(format!("{}", min_sales));
        }
        if let Some(max_sales) = query.max_sales {
            data_builder.push(" AND sales <= ");
            data_builder.push_bind(format!("{}", max_sales));
            count_builder.push(" AND sales <= ");
            count_builder.push_bind(format!("{}", max_sales));
        }
        if let Some(min_publish_date) = query.min_publish_date {
            data_builder.push(" AND publish_date >= ");
            data_builder.push_bind(format!("{}", min_publish_date));
            count_builder.push(" AND publish_date >= ");
            count_builder.push_bind(format!("{}", min_publish_date));
        }
        if let Some(max_publish_date) = query.max_publish_date {
            data_builder.push(" AND publish_date <= ");
            data_builder.push_bind(format!("{}", max_publish_date));
            count_builder.push(" AND publish_date <= ");
            count_builder.push_bind(format!("{}", max_publish_date));
        }
        data_builder.push(" ORDER BY publish_date DESC LIMIT  ");
        data_builder.push_bind(page_size);
        data_builder.push(" OFFSET ");
        data_builder.push_bind(offset);

        println!("SQL-data: {}", data_builder.sql());
        println!("SQL-count: {}", count_builder.sql());
        println!("Page size: {}, Offset: {}", page_size, offset);

        let books = data_builder.build_query_as().fetch_all(&mut *pool.acquire().await?).await.context("Failed to search books")?;
        let total: i32 = count_builder.build_query_scalar().fetch_one(&mut *pool.acquire().await?).await.context("Failed to get search count")?;

        Ok(Page {
            data: books,
            page_count: ((total as f32) / (page_size as f32)).ceil() as i32,
            current_page: page,
            total,
            page_size,
        })
    }

    // 创建查询条件
    // let query = BookQuery {
    //     title: Some("Rust".to_string()),
    //     min_rating: Some(4),
    //     category: Some("编程".to_string()),
    //     ..Default::default()
    // };

    // // 执行搜索
    // let results = search(&query, 1, 10).await?;

    /// 获取数据库连接池
    async fn get_pool() -> Result<MySqlPool> {
        let guard = POOL.lock().await;
        let pool = guard.as_ref().context("Database connection not initialized")?.clone();
        Ok(pool)
    }
}
