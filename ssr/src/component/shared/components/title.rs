use leptos::*;

#[component]
pub fn Title(
    /// `children` takes the `Children` type
    /// this is an alias for `Box<dyn FnOnce() -> Fragment>`
    #[prop(default = true)]
    justify_center: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class="sticky top-0 bg-transparent text-white p-4 w-full items-center z-50"
            class:justify-center=justify_center
            class:flex=justify_center
        >
            {children()}
        </span>
    }
}
