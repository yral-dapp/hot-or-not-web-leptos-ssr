use super::spinner::Spinner;
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

                class="flex overflow-hidden fixed top-0 left-0 justify-center items-center cursor-pointer modal-bg w-dvw h-dvh bg-black/60 z-[99]"
            >
                {children()}
            </div>
        </Show>
    }
}

#[component]
fn ActionRunningOverlay(message: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 justify-center items-center w-full h-full text-xl font-semibold text-center text-white">
            <Spinner/>
            <span>{message}</span>
            <span>Please wait...</span>
        </div>
    }
}

/// Tracks an action's progress and shows a modal with the result
/// action -> The action to track
/// loading_message -> The message to show while the action is pending
/// modal -> The modal to show when the action is done
/// close -> Set this signal to true to close the modal (automatically reset upon closing)
#[component]
pub fn PopupOverlay<
    S: 'static,
    R: 'static + Clone,
    V: IntoView,
    IV: Fn(R) -> V + Clone + 'static,
>(
    action: Action<S, R>,
    #[prop(into)] loading_message: String,
    modal: IV,
    #[prop(optional, into)] close: RwSignal<bool>,
) -> impl IntoView {
    let pending = action.pending();
    let action_value = action.value();
    let res = Signal::derive(move || {
        if pending() {
            return None;
        }
        action_value()
    });
    let show_popup = Signal::derive(move || {
        let show = (pending() || res.with(|r| r.is_some())) && !close();
        close.set_untracked(false);
        show
    });
    let modal_s = store_value(modal);
    let loading_msg_s = store_value(loading_message);

    view! {
        <ShadowOverlay show=show_popup>
            <Show
                when=move || res.with(|r| r.is_some())
                fallback=move || view! { <ActionRunningOverlay message=loading_msg_s.get_value()/> }
            >
                <div class="px-4 pt-4 pb-12 mx-6 w-full lg:w-1/2 max-h-[65%] rounded-xl bg-white">
                    {move || (modal_s.get_value())(res().unwrap())}
                </div>
            </Show>
        </ShadowOverlay>
    }
}
