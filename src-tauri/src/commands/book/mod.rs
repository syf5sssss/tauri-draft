use crate::commands::envpath::env_path;
use crate::dao::BookDao;
use crate::dto::{ Book, BookQuery, Page };
use chrono::Local;
use std::fs;
use std::path::PathBuf;
use tauri::{ command, AppHandle };
use uuid::Uuid;

#[command]
pub fn greetbook() -> String {
    let local_time = Local::now();
    let utc_time = local_time.naive_local();
    let bk = Book {
        id: Some(1),
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
    let utc_time = local_time.naive_local();
    let bks = [
        Book {
            id: Some(1),
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
            id: Some(2),
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
        Ok(books) => Ok(books),
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
        Ok(pagebooks) => Ok(pagebooks),
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
        Ok(pagebooks) => Ok(pagebooks),
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
pub async fn create(book: Book) -> Result<String, String> {
    let res = BookDao::create(&book).await.map_err(|e| format!("创建书籍失败: {}", e))?;
    Ok(format!("{:?}", res))
}

#[command]
pub async fn update(book: Book) -> Result<String, String> {
    match BookDao::update(&book).await {
        Ok(b) => Ok(b.to_string()),
        Err(e) => {
            let error_message = format!("Failed to update books: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
    // let res = BookDao::update(&book).await.map_err(|e| format!("更新书籍失败: {}", e))?;
    // Ok(format!("{:?}", res))
}
#[command]
pub async fn delete(id: i32) -> Result<String, String> {
    match BookDao::delete(id.try_into().unwrap()).await {
        Ok(b) => Ok(b.to_string()),
        Err(e) => {
            let error_message = format!("Failed to delete books: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
}

#[command]
pub async fn deletes(ids: Vec<u32>) -> Result<String, String> {
    match BookDao::deletes(&ids).await {
        Ok(b) => Ok(b.to_string()),
        Err(e) => {
            let error_message = format!("Failed to delete books: {:?}", e);
            println!("{}", error_message);

            if let Some(db_err) = e.downcast_ref::<sqlx::Error>() {
                println!("Database error: {:?}", db_err);
            }

            Err(error_message)
        }
    }
}

#[command]
pub async fn save_image(app: AppHandle, bytes: Vec<u8>) -> Result<String, String> {
    match env_path::get_pictures_dir(app) {
        Ok(dir) => {
            println!("当前运行目录的地址: {:?}", dir);
            let base_dir = PathBuf::from(dir);
            let target_dir = base_dir.join("Draft").join("Img");
            let uuid = Uuid::new_v4();
            let file_name = uuid.to_string().replace("-", "") + ".png";
            if let Err(e) = fs::create_dir_all(&target_dir) {
                return Err(format!("创建目录失败: {}", e));
            }
            // 构建完整文件路径
            let file_path = target_dir.join(&file_name);
            // 写入文件
            match fs::write(&file_path, &bytes) {
                Ok(_) => {
                    println!("图片已保存至: {}", file_path.display());
                    Ok(file_name)
                }
                Err(e) => Err(format!("写入文件失败: {}", e)),
            }
        }
        Err(_e) => {
            return Err(format!("无法获取应用运行目录"));
        }
    }
}
