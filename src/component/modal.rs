use leptos::*;
use leptos_icons::*;

#[component]
pub fn Modal(#[prop(into)] show: RwSignal<bool>, children: Children) -> impl IntoView {
    view! {
        <div
            on:click={
                #[cfg(feature = "hydrate")]
                {
                    move |ev| {
                        use web_sys::HtmlElement;
                        let target = event_target::<HtmlElement>(&ev);
                        if target.class_list().contains("modal-bg") {
                            show.set(false);
                        }
                    }
                }
                #[cfg(not(feature = "hydrate"))] { |_| () }
            }

            class="cursor-pointer modal-bg w-screen h-screen absolute left-0 top-0 bg-black/60 z-[99] justify-center items-center"
            style:display=move || if show() { "flex" } else { "none" }
        >
            <div class="w-2/5 h-2/5 cursor-auto flex-col flex justify-around bg-neutral-900 rounded-md divide-y-2 divide-neutral-800">
                <div class="flex w-full justify-end py-2 px-5">
                    <button
                        on:click=move |_| show.set(false)
                        class="text-white text-center p-1 text-xl bg-orange-600  rounded-full"
                    >
                        <Icon icon=icondata::ChCross/>
                    </button>
                </div>
                {children()}
            </div>
        </div>
    }
}
