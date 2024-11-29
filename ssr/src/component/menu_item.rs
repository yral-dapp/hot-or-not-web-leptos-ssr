use leptos::{component, view, Callback, IntoView};
use leptos_icons::*;

#[component]
pub fn MenuLink(
    #[prop(into)] text: String,
    #[prop(into)] href: String,
    #[prop(into)] icon: icondata::Icon,
    #[prop(into, optional)] target: String,
) -> impl IntoView {
    view! {
        <a href=href class="grid grid-cols-3 items-center w-full" target=target>
            <div class="flex flex-row gap-4 items-center col-span-2">
                <Icon class="text-2xl" icon=icon />
                <span class="text-wrap">{text}</span>
            </div>
            <Icon class="text-2xl justify-self-end" icon=icondata::AiRightOutlined />
        </a>
    }
}

#[component]
pub fn MenuButton(
    #[prop(into)] text: String,
    #[prop(into)] icon: icondata::Icon,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button class="grid grid-cols-3 items-center w-full" on:click=move |_| on_click(())>
            <div class="flex flex-row gap-4 items-center col-span-2">
                <Icon class="text-2xl" icon=icon />
                <span class="text-wrap">{text}</span>
            </div>
            <Icon class="text-2xl justify-self-end" icon=icondata::AiRightOutlined />
        </button>
    }
}
