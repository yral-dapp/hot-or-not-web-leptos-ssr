use leptos::*;

#[component]
pub fn Loading(text: String, children: Children) -> impl IntoView {
    view! {
        {children()}
        <div class="h-dvh w-dvw bg-black flex flex-col justify-center items-center gap-10">
            <img class="h-56 w-56 object-contain animate-pulse" src="/img/logo.webp" />
            <span class="text-2xl text-white/60">{text}</span>
        </div>
    }
}
