pub mod db;
pub mod general;

pub use db::DbState;
pub use db::{
    add_baptism, add_child, add_family, add_work, delete_baptisms, delete_families, delete_works,
    get_baptisms, get_children_by_family, get_families, get_spouse, get_works, list_tables,
    save_children, save_spouse,
};
pub use general::exit_app;
