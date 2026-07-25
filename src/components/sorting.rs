use yew::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub enum SortDir {
    #[default]
    Asc,
    Desc,
}

#[derive(Default, Clone, PartialEq)]
pub struct SortState<T: Default> {
    pub column: T,
    pub dir: SortDir,
}

impl<T: Default + Clone + PartialEq> SortState<T> {
    pub fn toggle(&self, col: T) -> Self {
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

pub fn sort_icon(active: bool, dir: &SortDir) -> Html {
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

#[macro_export]
macro_rules! on_sort {
    ($col:expr, $sort:expr) => {{
        let sort = $sort.clone();
        Callback::from(move |_: MouseEvent| sort.set((*sort).clone().toggle($col)))
    }};
}
