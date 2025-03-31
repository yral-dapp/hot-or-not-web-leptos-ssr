use super::overlay::ShadowOverlay;
use leptos::prelude::*;
use leptos_icons::*;

#[component]
pub fn Modal(#[prop(into)] show: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    view! {
        <ShadowOverlay show>
            <div class="mx-4 py-4 px-[20px] max-w-full max-h-full items-center cursor-auto flex-col flex justify-around bg-neutral-900 rounded-md">
                <div class="flex w-full justify-end items-center">
                    <button
                        on:click=move |_| show.set(false)
                        class="text-white text-center p-1 text-lg md:text-xl bg-neutral-600 rounded-full"
                    >
                        <Icon icon=icondata::ChCross />
                    </button>
                </div>
                <div class="pb-4 w-full">{children()}</div>
            </div>
        </ShadowOverlay>
    }.into_any()
}
