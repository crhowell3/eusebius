pub mod db;
pub mod general;

pub use db::DbState;
pub use db::{
    add_child, add_spouse, add_work, delete_works, get_children, get_families, get_spouse,
    get_works,
};
pub use general::exit_app;
