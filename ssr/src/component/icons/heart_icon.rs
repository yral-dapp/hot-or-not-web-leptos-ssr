use leptos::*;

#[component]
pub fn HeartIcon(
    #[prop(into, optional, default = "w-full h-full".into())] class: String,
) -> impl IntoView {
    view! {
      <svg class=format!("{}", class) viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M18.8 29.9a3 3 0 0 1-1.6 0c-4-1.4-12.7-7-12.7-16.4A7.5 7.5 0 0 1 18 9a7.5 7.5 0 0 1 13.5 4.5c0 9.5-8.7 15-12.7 16.4Z" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    }
}