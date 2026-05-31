use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Work {
    pub work_code: String,
    pub description: String,
}

pub enum GenericAction {
    SetField { name: String, value: String },
    Reset,
}

impl yew::prelude::Reducible for Work {
    type Action = GenericAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            GenericAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "work_code" => updated.work_code = value,
                    "description" => updated.description = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Work::default()),
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

#[derive(Clone, Serialize, Deserialize, Default)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Child {
    pub family_id: String,
    pub first_name: String,
    pub last_name: String,
}

impl Child {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Spouse {
    pub family_id: String,
    pub first_name: String,
    pub last_name: String,
}

impl Spouse {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
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
        Self::default()
    }
}

impl yew::prelude::Reducible for Family {
    type Action = GenericAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            GenericAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "family_id" => updated.family_id = value,
                    "mail_route" => updated.mail_route = value,
                    "last_name" => updated.last_name = value,
                    "first_name" => updated.first_name = value,
                    //"is_member" => updated.is_member = value,
                    //"is_active" => updated.is_active = value,
                    "date_of_birth" => updated.date_of_birth = value,
                    "anniversary_month" => updated.anniversary_month = value,
                    "anniversary_day" => updated.anniversary_day = value,
                    "home_phone" => updated.home_phone = value,
                    "cell_phone" => updated.cell_phone = value,
                    "work_phone" => updated.work_phone = value,
                    "address" => updated.address = value,
                    "city" => updated.city = value,
                    "state" => updated.state = value,
                    "zip" => updated.zip = value,
                    "email_address" => updated.email_address = value,
                    //"on_bulletin_email_list" => updated.on_bulletin_email_list = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Family::default()),
        }
    }
}
