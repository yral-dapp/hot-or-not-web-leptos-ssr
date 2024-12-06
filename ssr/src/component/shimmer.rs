use leptos::*;

#[component]
pub fn Shimmer() -> impl IntoView {
    view! {
        <div
            style="background: linear-gradient(-45deg, rgb(24, 24, 27) 40%, rgb(20, 20, 20) 50%, rgb(24, 24, 27) 60%); background-size: 300%; background-position-x: 100%;"
            class="w-full h-8 bg-transparent animate-shimmer bg-zinc-900 rounded-full"
        />
    }
}
