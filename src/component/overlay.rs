use leptos::*;

#[component]
pub fn ShadowOverlay(#[prop(into)] show: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    view! {
        <Show when=show>
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

                class="flex cursor-pointer modal-bg w-dvw h-dvh fixed left-0 top-0 bg-black/60 z-[99] justify-center items-center overflow-hidden"
            >
                {children()}
            </div>
        </Show>
    }
}
