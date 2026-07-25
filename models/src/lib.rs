use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub font_size: u32,
    pub confirm_before_delete: bool,
    pub rows_per_page: u32,
    pub max_backups: u32,
    pub backup_on_startup: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            font_size: 16,
            confirm_before_delete: true,
            rows_per_page: 50,
            max_backups: 10,
            backup_on_startup: false,
        }
    }
}

pub enum GenericAction {
    SetField { name: String, value: String },
    SetBool { name: String, value: bool },
    Reset,
}

pub enum SpouseAction {
    SetField { name: String, value: String },
    SetBool { name: String, value: bool },
    Reset,
    Load(Spouse),
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct TableInfo {
    pub name: String,
    pub path: String,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Work {
    pub id: i64,
    pub description: String,
    pub category_id: i64,
    pub category_tag: String,
    pub category_name: String,
}

impl yew::prelude::Reducible for Work {
    type Action = GenericAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            GenericAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "description" => updated.description = value,
                    "category_id" => {
                        if let Ok(id) = value.parse::<i64>() {
                            updated.category_id = id;
                        }
                    }
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::SetBool { name: _, value: _ } => {
                let updated = (*self).clone();
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Work::default()),
        }
    }
}

fn today_string() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Baptism {
    pub family_id: String,
    pub last_name: String,
    pub first_name: String,
    pub date_baptized: String,
    pub witness: String,
    pub location: String,
}

impl Default for Baptism {
    fn default() -> Self {
        Self {
            family_id: String::new(),
            last_name: String::new(),
            first_name: String::new(),
            date_baptized: today_string(),
            witness: String::new(),
            location: String::new(),
        }
    }
}

impl yew::prelude::Reducible for Baptism {
    type Action = GenericAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            GenericAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "family_id" => updated.family_id = value,
                    "last_name" => updated.last_name = value,
                    "first_name" => updated.first_name = value,
                    "date_baptized" => updated.date_baptized = value,
                    "witness" => updated.witness = value,
                    "location" => updated.location = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::SetBool { name: _, value: _ } => {
                let updated = (*self).clone();
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Baptism::default()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Child {
    pub id: i64,
    pub family_id: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub is_member: bool,
    pub is_active: bool,
    pub cell_phone: String,
    pub work_phone: String,
    pub email_address: String,
    pub on_bulletin_email_list: bool,
}

impl Child {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl yew::prelude::Reducible for Child {
    type Action = GenericAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            GenericAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "family_id" => updated.family_id = value,
                    "last_name" => updated.last_name = value,
                    "first_name" => updated.first_name = value,
                    "date_of_birth" => updated.date_of_birth = value,
                    "cell_phone" => updated.cell_phone = value,
                    "work_phone" => updated.work_phone = value,
                    "email_address" => updated.email_address = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::SetBool { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "is_member" => updated.is_member = value,
                    "is_active" => updated.is_active = value,
                    "on_bulletin_email_list" => updated.on_bulletin_email_list = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Child::default()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Spouse {
    pub family_id: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub is_member: bool,
    pub is_active: bool,
    pub cell_phone: String,
    pub work_phone: String,
    pub email_address: String,
    pub on_bulletin_email_list: bool,
}

impl Spouse {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl yew::prelude::Reducible for Spouse {
    type Action = SpouseAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            SpouseAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "family_id" => updated.family_id = value,
                    "last_name" => updated.last_name = value,
                    "first_name" => updated.first_name = value,
                    "date_of_birth" => updated.date_of_birth = value,
                    "cell_phone" => updated.cell_phone = value,
                    "work_phone" => updated.work_phone = value,
                    "email_address" => updated.email_address = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            SpouseAction::SetBool { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "is_member" => updated.is_member = value,
                    "is_active" => updated.is_active = value,
                    "on_bulletin_email_list" => updated.on_bulletin_email_list = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            SpouseAction::Reset => std::rc::Rc::new(Spouse::default()),
            SpouseAction::Load(existing) => std::rc::Rc::new(existing),
        }
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
    #[must_use]
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
                    "mail_route" => updated.mail_route = value,
                    "last_name" => updated.last_name = value,
                    "first_name" => updated.first_name = value,
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
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::SetBool { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "is_member" => updated.is_member = value,
                    "is_active" => updated.is_active = value,
                    "on_bulletin_email_list" => updated.on_bulletin_email_list = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Family::default()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Death {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub date_of_death: String,
}

impl Death {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl yew::prelude::Reducible for Death {
    type Action = GenericAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            GenericAction::SetField { name, value } => {
                let mut updated = (*self).clone();
                match name.as_str() {
                    "last_name" => updated.last_name = value,
                    "first_name" => updated.first_name = value,
                    "date_of_death" => updated.date_of_death = value,
                    _ => {}
                }
                std::rc::Rc::new(updated)
            }
            GenericAction::SetBool { name: _, value: _ } => {
                let updated = (*self).clone();
                std::rc::Rc::new(updated)
            }
            GenericAction::Reset => std::rc::Rc::new(Death::default()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Category {
    pub id: i64,
    pub tag: String,
    pub name: String,
}

impl Category {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

pub enum CategoryAction {
    SetName(String),
    SetTag(String),
    Reset,
}

impl yew::prelude::Reducible for Category {
    type Action = CategoryAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CategoryAction::SetName(name) => std::rc::Rc::new(Self {
                id: self.id,
                tag: self.tag.clone(),
                name,
            }),
            CategoryAction::SetTag(tag) => std::rc::Rc::new(Self {
                id: self.id,
                tag,
                name: self.name.clone(),
            }),
            CategoryAction::Reset => std::rc::Rc::new(Category::default()),
        }
    }
}
