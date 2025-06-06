use chrono::{ Local, Utc };
use tauri::command;
use crate::dto::{ Book, BookQuery, Page };
use crate::dao::BookDao;

#[command]
pub fn greetbook() -> String {
    let local_time = Local::now();
    let utc_time = local_time.with_timezone(&Utc);
    let bk = Book {
        id: 1,
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik".to_string(),
        category: "Programming".to_string(),
        price: 39.98,
        sales: 1000000,
        publish_date: utc_time,
        rating: 4,
        img: "/demo/images/draft/1.jpg".to_string(),
        status: "LOWSTOCK".to_string(),
    };
    format!("{:?}", bk)
}

#[command]
pub fn greetbooks() -> String {
    let local_time = Local::now();
    let utc_time = local_time.with_timezone(&Utc);
    let bks = [
        Book {
            id: 1,
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            category: "Programming".to_string(),
            price: 67.98,
            sales: 2000,
            publish_date: utc_time,
            rating: 4,
            img: "/demo/images/draft/2.jpg".to_string(),
            status: "OUTOFSTOCK".to_string(),
        },
        Book {
            id: 2,
            title: "The Tauri Programming Language".to_string(),
            author: "BoBo Klabnik".to_string(),
            category: "Programmed".to_string(),
            price: 12.78,
            sales: 6000,
            publish_date: utc_time,
            rating: 4,
            img: "/demo/images/draft/3.jpg".to_string(),
            status: "INSTOCK".to_string(),
        },
    ];
    format!("{:?}", bks)
}

#[command]
pub async fn list() -> Result<Vec<Book>, String> {
    match BookDao::list(1, 10).await {
        Ok(books) => { Ok(books) }
        Err(e) => {
            let error_message = format!("Failed to list books: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
}

#[command]
pub async fn search(query: &str, current_page: i32, page_size: i32) -> Result<Page<Book>, String> {
    match BookDao::search(query, current_page, page_size).await {
        Ok(pagebooks) => { Ok(pagebooks) }
        Err(e) => {
            let error_message = format!("Failed to list books: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
}

#[command]
pub async fn dynamics_search(query: BookQuery, current_page: i32, page_size: i32) -> Result<Page<Book>, String> {
    match BookDao::dynamics_search(&query, current_page, page_size).await {
        Ok(pagebooks) => { Ok(pagebooks) }
        Err(e) => {
            let error_message = format!("Failed to list books: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
}
