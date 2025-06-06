// src/commands/mod.rs
pub mod greet;
pub mod book;
pub mod draft;
pub mod sqlx;

// 导出所有命令函数
pub use greet::*;
pub use book::*;
pub use draft::*;
pub use sqlx::*;

// 生成最终的处理函数
#[macro_export]
macro_rules! register_commands {
    () => {
        tauri::generate_handler![
            $crate::commands::greet::greet,
            $crate::commands::draft::multiplication99,
            $crate::commands::book::greetbook,
            $crate::commands::book::greetbooks,
            $crate::commands::book::list,
            $crate::commands::book::search,
            $crate::commands::book::dynamics_search,
            $crate::commands::sqlx::connect_db,
            $crate::commands::sqlx::get_alldbname,
            $crate::commands::sqlx::get_alltablenamebydbname,
            $crate::commands::sqlx::get_table_columns,
            $crate::commands::sqlx::execute_sql_command,
            $crate::commands::sqlx::query_table_data,
            $crate::commands::sqlx::backup_db_command,
            $crate::commands::sqlx::revert_db_command,
            $crate::commands::sqlx::execute_dbtable_command,
            $crate::commands::sqlx::check_and_create_traft_db
        ]
    };
}

pub use register_commands;
