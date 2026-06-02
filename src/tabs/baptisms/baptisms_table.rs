use std::collections::HashSet;

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::invoke;
use shared::Baptism;

#[derive(Properties, PartialEq)]
pub struct BaptismsTableProps {
    pub refresh_trigger: u32,
}

async fn fetch_baptisms() -> Result<Vec<Baptism>, String> {
    let result = invoke("get_baptisms", JsValue::UNDEFINED).await;
    from_value::<Vec<Baptism>>(result).map_err(|e| e.to_string())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteBaptismsArgs {
    family_ids: Vec<String>,
}

async fn delete_baptisms(family_ids: Vec<String>) -> Result<(), String> {
    let args = to_value(&DeleteBaptismsArgs { family_ids }).map_err(|e| e.to_string())?;
    let result = invoke("delete_baptisms", args).await;
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        Err(result.as_string().unwrap_or("Unknown error".to_string()))
    }
}

#[derive(Clone, PartialEq)]
enum SortColumn {
    FamilyId,
    Description,
}

#[derive(Clone, PartialEq)]
enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone, PartialEq)]
struct SortState {
    column: SortColumn,
    dir: SortDir,
}

impl SortState {
    fn default() -> Self {
        Self {
            column: SortColumn::FamilyId,
            dir: SortDir::Asc,
        }
    }

    fn toggle(&self, col: SortColumn) -> Self {
        if self.column == col {
            Self {
                column: col,
                dir: match self.dir {
                    SortDir::Asc => SortDir::Desc,
                    SortDir::Desc => SortDir::Asc,
                },
            }
        } else {
            Self {
                column: col,
                dir: SortDir::Asc,
            }
        }
    }
}

fn sort_baptisms(baptisms: &[Baptism], sort: &SortState) -> Vec<Baptism> {
    let mut sorted = baptisms.to_vec();
    sorted.sort_by(|a, b| {
        let ord = match sort.column {
            SortColumn::FamilyId => a.family_id.cmp(&b.family_id),
            SortColumn::Description => a.last_name.cmp(&b.last_name),
        };
        match sort.dir {
            SortDir::Asc => ord,
            SortDir::Desc => ord.reverse(),
        }
    });
    sorted
}

fn sort_icon(active: bool, dir: &SortDir) -> Html {
    if !active {
        return html! {
            <svg class="sort-icon sort-icon--inactive" xmlns="http://www.w3.org/2000/svg"
                width="12" height="12" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2"
                stroke-linecap="round" stroke-linejoin="round">
                <path d="M7 15l5 5 5-5"/>
                <path d="M7 9l5-5 5 5"/>
            </svg>
        };
    }
    match dir {
        SortDir::Asc => html! {
            <svg class="sort-icon sort-icon--active" xmlns="http://www.w3.org/2000/svg"
                width="12" height="12" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2.5"
                stroke-linecap="round" stroke-linejoin="round">
                <path d="M7 15l5 5 5-5"/>
            </svg>
        },
        SortDir::Desc => html! {
            <svg class="sort-icon sort-icon--active" xmlns="http://www.w3.org/2000/svg"
                width="12" height="12" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2.5"
                stroke-linecap="round" stroke-linejoin="round">
                <path d="M7 9l5-5 5 5"/>
            </svg>
        },
    }
}

#[function_component(BaptismsTable)]
pub fn baptisms_table(props: &BaptismsTableProps) -> Html {
    let baptisms = use_state(Vec::new);
    let error = use_state(|| None::<String>);
    let selected = use_state(HashSet::<String>::new);
    let initial_load = use_state(|| true);
    let sort = use_state(SortState::default);

    {
        let baptisms = baptisms.clone();
        let error = error.clone();
        let selected = selected.clone();
        let initial_load = initial_load.clone();
        let trigger = props.refresh_trigger;

        use_effect_with(trigger, move |_| {
            spawn_local(async move {
                match fetch_baptisms().await {
                    Ok(data) => {
                        baptisms.set(data);
                        error.set(None);
                        selected.set(HashSet::new());
                    }
                    Err(e) => error.set(Some(e)),
                }
                initial_load.set(false);
            });
            || ()
        });
    }

    let sorted_baptisms = sort_baptisms(&*baptisms, &*sort);

    let all_checked = !sorted_baptisms.is_empty()
        && sorted_baptisms
            .iter()
            .all(|b| (*selected).contains(&b.family_id));
    let some_checked = !(*selected).is_empty();
    let selected_count = (*selected).len();

    let on_sort_family_id = {
        let sort = sort.clone();
        Callback::from(move |_: MouseEvent| sort.set((*sort).clone().toggle(SortColumn::FamilyId)))
    };

    html! {
        <section class="works-table-card">

            <div class="works-table-toolbar">
                <div class="works-table-toolbar-left">
                    <h3 class="works-table-title">{ "Baptism Records" }</h3>
                    if !sorted_baptisms.is_empty() {
                        <span class="works-table-count">
                            { format!("{} record{}", sorted_baptisms.len(),
                                if sorted_baptisms.len() == 1 { "" } else { "s" }) }
                        </span>
                    }
                </div>
            </div>
        </section>
    }
}
