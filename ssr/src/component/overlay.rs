use leptos::*;

#[derive(Clone, Copy)]
pub enum ShowOverlay {
    /// Show overlay and allow closing by user
    Closable(RwSignal<bool>),
    MaybeClosable {
        show: RwSignal<bool>,
        /// Allow closing based on this signal
        closable: RwSignal<bool>,
    },
    /// Show overlay but prevent closing by user
    AlwaysLocked(Signal<bool>),
}

impl From<bool> for ShowOverlay {
    fn from(b: bool) -> Self {
        ShowOverlay::Closable(RwSignal::new(b))
    }
}

impl From<RwSignal<bool>> for ShowOverlay {
    fn from(s: RwSignal<bool>) -> Self {
        ShowOverlay::Closable(s)
    }
}

impl From<Signal<bool>> for ShowOverlay {
    fn from(s: Signal<bool>) -> Self {
        ShowOverlay::AlwaysLocked(s)
    }
}

impl SignalGet for ShowOverlay {
    type Value = bool;

    fn get(&self) -> bool {
        match self {
            ShowOverlay::Closable(s) => s.get(),
            ShowOverlay::AlwaysLocked(s) => s.get(),
            ShowOverlay::MaybeClosable { show, .. } => show.get(),
        }
    }

    fn try_get(&self) -> Option<bool> {
        match self {
            ShowOverlay::Closable(s) => s.try_get(),
            ShowOverlay::AlwaysLocked(s) => s.try_get(),
            ShowOverlay::MaybeClosable { show, .. } => show.try_get(),
        }
    }
}

impl SignalSet for ShowOverlay {
    type Value = bool;

    fn set(&self, value: bool) {
        match self {
            ShowOverlay::Closable(s) => s.set(value),
            ShowOverlay::AlwaysLocked(_) => {}
            ShowOverlay::MaybeClosable { show, closable } => {
                if closable.get_untracked() {
                    show.set(value);
                }
            }
        }
    }

    fn try_set(&self, value: bool) -> Option<bool> {
        match self {
            ShowOverlay::Closable(s) => s.try_set(value),
            ShowOverlay::AlwaysLocked(_) => None,
            ShowOverlay::MaybeClosable { show, closable } => {
                if closable.try_get_untracked()? {
                    show.try_set(value)
                } else {
                    None
                }
            }
        }
    }
}

#[component]
pub fn ShadowOverlay(#[prop(into)] show: ShowOverlay, children: ChildrenFn) -> impl IntoView {
    view! {
        <Show when=move || show.get()>
            <div
                on:click={
                    #[cfg(feature = "hydrate")]
                    {
                        move |ev| {
                            use web_sys::HtmlElement;
                            let target = event_target::<HtmlElement>(&ev);
                            if !target.class_list().contains("modal-bg") {
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
