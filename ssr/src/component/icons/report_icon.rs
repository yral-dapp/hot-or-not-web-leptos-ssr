use leptos::*;

#[component]
pub fn ReportIcon(
    #[prop(into, optional, default = "w-full h-full".into())] class: String,
) -> impl IntoView {
    view! {
      <svg class=format!("{}", class) viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M18 13.5v3m0 6h0m-10.4 6h20.8A3 3 0 0 0 31 24L20.6 6a3 3 0 0 0-5.2 0L5 24a3 3 0 0 0 2.6 4.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    }
}
