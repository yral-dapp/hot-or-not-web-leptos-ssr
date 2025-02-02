use leptos::{component, view, IntoView};
use leptos_icons::*;

/// a dumb tooltip. Can't specify direction, customize content, make it stick with a close button, etc.
#[component]
pub fn Tooltip(
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    let _ = title;
    view! {
        <div class="relative group">
            <div class="tooltip-target bg-neutral-800 grid place-items-center size-[22px] rounded-full cursor-pointer">
                <Icon class="size-[22px]" icon=icon />
            </div>
            <div class="w-max max-w-[85vw] md:max-w-[400px] absolute pointer-events-none duration-150 rounded-md top-0 mt-8 z-50 opacity-0 group-hover:opacity-100 bg-[#EAC9DB] text-[#A00157] p-4 -ml-1">
                <div class="absolute mr-1 -translate-x-1/2 bottom-full h-0 w-0 border-r-4 border-l-4 border-b-4 border-l-transparent border-r-transparent border-b-[#EAC9DB]"></div>
                <h2 class="font-bold">{title}</h2>
                <div>
                    {description}
                </div>
            </div>
        </div>
    }
}
