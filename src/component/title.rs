use leptos::*;
use leptos_icons::*;

#[component]
pub fn Title(
    /// `children` takes the `Children` type
    /// this is an alias for `Box<dyn FnOnce() -> Fragment>`
    children: Children,
) -> impl IntoView {
    view! {
        <span class="sticky top-0 bg-black text-white p-4 w-screen flex flex-col justify-center items-center z-50">
            {children()}
        </span>
    }
}
