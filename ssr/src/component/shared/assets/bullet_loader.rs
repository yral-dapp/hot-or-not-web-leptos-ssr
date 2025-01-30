use leptos::*;

#[component]
pub fn BulletLoader() -> impl IntoView {
    view! {
        <div class="flex justify-center h-full w-full basis-full">
            <div class="flex flex-row gap-2">
                <div class="w-4 h-4 rounded-full bg-white/50 animate-bounce"></div>
                <div
                    class="w-4 h-4 rounded-full bg-white/50 animate-bounce"
                    style:animation-delay="-300ms"
                ></div>
                <div
                    class="w-4 h-4 rounded-full bg-white/50 animate-bounce"
                    style:animation-delay="-500ms"
                ></div>
                <div
                    class="w-4 h-4 rounded-full bg-white/50 animate-bounce"
                    style:animation-delay="-700ms"
                ></div>
            </div>
        </div>
    }
}
