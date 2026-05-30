use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Work {
    pub work_code: String,
    pub description: String,
}

impl Default for Work {
    fn default() -> Self {
        Self {
            work_code: String::new(),
            description: String::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Child {
    pub family_id: String,
    pub first_name: String,
    pub last_name: String,
}

impl Child {
    pub fn new() -> Self {
        Self {
            family_id: String::new(),
            first_name: String::new(),
            last_name: String::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Spouse {
    pub family_id: String,
    pub first_name: String,
    pub last_name: String,
}

impl Spouse {
    pub fn new() -> Self {
        Self {
            family_id: String::new(),
            first_name: String::new(),
            last_name: String::new(),
        }
    }
}
