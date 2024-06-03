use leptos::*;

use crate::utils::report::ReportOption;

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value() == is
        >
            {is}
        </option>
    }
}
