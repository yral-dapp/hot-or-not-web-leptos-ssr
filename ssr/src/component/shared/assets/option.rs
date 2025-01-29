use leptos::*;

#[component]
pub fn SelectOption(#[prop(into)] is: String, value: ReadSignal<String>) -> impl IntoView {
    let is_copy = is.clone();

    view! {
        <option value=is.clone() selected=move || value() == is_copy>
            {is}
        </option>
    }
}
