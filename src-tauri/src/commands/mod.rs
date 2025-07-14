pub mod book;
pub mod config;
pub mod draft;
pub mod envpath;
pub mod fileio;
pub mod greet;
pub mod listmap;
pub mod popula;
pub mod sqlx;
pub mod tcp;
pub mod thread;
pub mod udp;
pub mod blazeface;

// 导出所有命令函数
pub use book::*;
pub use config::*;
pub use draft::*;
pub use envpath::*;
pub use fileio::*;
pub use greet::*;
pub use listmap::*;
pub use popula::*;
pub use sqlx::*;
pub use tcp::*;
pub use thread::*;
pub use udp::*;
pub use blazeface::*;

// 生成最终的处理函数
#[macro_export]
macro_rules! register_commands {
    () => {
        tauri::generate_handler![
            $crate::commands::greet::greet,
            $crate::commands::greet::log_from_js,
            $crate::commands::draft::multiplication99,
            $crate::commands::book::greetbook,
            $crate::commands::book::greetbooks,
            $crate::commands::book::list,
            $crate::commands::book::search,
            $crate::commands::book::dynamics_search,
            $crate::commands::book::create,
            $crate::commands::book::update,
            $crate::commands::book::delete,
            $crate::commands::book::deletes,
            $crate::commands::book::save_image,
            $crate::commands::env_path::get_exe_dir,
            $crate::commands::env_path::get_pictures_dir,
            $crate::commands::env_path::get_documents_dir,
            $crate::commands::config::get_config,
            $crate::commands::config::get_config_field,
            $crate::commands::config::set_config_field,
            $crate::commands::config::reset_config,
            $crate::commands::thread::start_time,
            $crate::commands::thread::stop_time,
            $crate::commands::thread::start_timer,
            $crate::commands::thread::stop_timer,
            $crate::commands::thread::get_shared_value,
            $crate::commands::thread::set_shared_value,
            $crate::commands::thread::get_last_update,
            $crate::commands::thread::progress_update,
            $crate::commands::tcp::start_tcp_server,
            $crate::commands::tcp::stop_tcp_server,
            $crate::commands::tcp::send_to_clients,
            $crate::commands::tcp::send_to_client,
            $crate::commands::tcp::get_connstr,
            $crate::commands::tcp::tcp_client_connect,
            $crate::commands::tcp::disconnect,
            $crate::commands::tcp::send_message,
            $crate::commands::udp::open_broadcast_service,
            $crate::commands::udp::close_broadcast_service,
            $crate::commands::udp::send_broadcast_message,
            $crate::commands::udp::open_udp_service,
            $crate::commands::udp::close_udp_service,
            $crate::commands::udp::send_udp_message,
            $crate::commands::udp::send_multicast_message,
            $crate::commands::fileio::rename_entries,
            $crate::commands::fileio::delete_entries,
            $crate::commands::fileio::create_entry,
            $crate::commands::fileio::get_directory_info,
            $crate::commands::fileio::delete_empty_directories,
            $crate::commands::fileio::open_entry_location,
            $crate::commands::fileio::move_entries,
            $crate::commands::listmap::init_students,
            $crate::commands::listmap::add_student,
            $crate::commands::listmap::delete_student,
            $crate::commands::listmap::update_student,
            $crate::commands::listmap::query_students,
            $crate::commands::listmap::get_student,
            $crate::commands::listmap::init_teachers,
            $crate::commands::listmap::add_teacher,
            $crate::commands::listmap::delete_teacher,
            $crate::commands::listmap::update_teacher,
            $crate::commands::listmap::query_teachers,
            $crate::commands::listmap::get_teacher,
            $crate::commands::popula::popula_list,
            $crate::commands::blazeface::face_init,
            $crate::commands::blazeface::write_file,
            $crate::commands::blazeface::cmd_detect_face_position,
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
