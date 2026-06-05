pub mod db;
pub mod general;

pub use db::DbState;
pub use db::{
    add_baptism, add_child, add_spouse, add_work, delete_baptisms, delete_works, get_baptisms,
    get_children, get_families, get_spouse, get_works, list_tables,
};
pub use general::exit_app;
