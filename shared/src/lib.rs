use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Work {
    pub work_code: String,
    pub description: String,
}

pub enum WorkAction {
    SetField { name: String, value: String },
    Reset,
}

impl yew::prelude::Reducible for Work {
    type Action = WorkAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            WorkAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "work_code" => updated.work_code = value,
                    "description" => updated.description = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            WorkAction::Reset => std::rc::Rc::new(Work::default()),
        }
    }
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

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Family {
    pub family_id: String,
    pub mail_route: String,
    pub last_name: String,
    pub first_name: String,
    pub is_member: bool,
    pub is_active: bool,
    pub date_of_birth: String,
    pub anniversary_month: String,
    pub anniversary_day: String,
    pub home_phone: String,
    pub cell_phone: String,
    pub work_phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub email_address: String,
    pub on_bulletin_email_list: bool,
}

impl Family {
    pub fn new() -> Self {
        Self {
            family_id: String::new(),
            mail_route: String::new(),
            last_name: String::new(),
            first_name: String::new(),
            is_member: false,
            is_active: false,
            date_of_birth: String::new(),
            anniversary_month: String::new(),
            anniversary_day: String::new(),
            home_phone: String::new(),
            cell_phone: String::new(),
            work_phone: String::new(),
            address: String::new(),
            city: String::new(),
            state: String::new(),
            zip: String::new(),
            email_address: String::new(),
            on_bulletin_email_list: false,
        }
    }
}
