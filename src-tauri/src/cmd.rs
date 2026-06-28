pub mod backup;
pub mod db;
pub mod general;
pub mod settings;

pub use backup::*;
pub use db::DbState;
pub use db::{
    add_baptism, add_child, add_death, add_family, add_work, delete_baptisms, delete_children,
    delete_deaths, delete_families, delete_works, get_baptisms, get_children_by_family, get_deaths,
    get_families, get_spouse, get_works, list_tables, save_children, save_spouse,
};
pub use general::exit_app;
pub use settings::*;
