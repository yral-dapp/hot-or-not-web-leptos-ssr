use leptos::*;

#[component]
pub fn HighlightedButton(
    children: Children,
    on_click: impl Fn() + 'static,
    #[prop(optional)] classes: String,
    #[prop(optional)] alt_style: bool,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let on_click = move |_| on_click();
    view! {
        <button
            on:click=on_click
            disabled=disabled
            class=format!(
                "w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold {}",
                classes,
            )
            style=format!(
                "background: linear-gradient(73deg, {} );",
                if alt_style {
                    "#FFF 0%, #FFF 1000%"
                } else {
                    "#FF78C1 0%, #E2017B 33%, #5F0938 100%"
                },
            )
        >
            <div class=move || {
                if alt_style{
                    "bg-gradient-to-r from-[#FF78C1] via-[#E2017B] to-[#5F0938] inline-block text-transparent bg-clip-text"
                } else {
                    "text-white"
                }
            }>{children()}</div>
        </button>
    }
}

#[component]
pub fn HighlightedLinkButton(
    children: Children,
    href: String,
    #[prop(optional)] classes: String,
    #[prop(optional)] alt_style: bool,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    view! {
        <a
            href=href
            disabled=disabled
            class=format!(
                "w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold {}",
                classes,
            )
            style=move || format!(
                "background: linear-gradient(73deg, {} );",
                if alt_style {
                    "#FFF 0%, #FFF 1000%"
                } else {
                    "#FF78C1 0%, #E2017B 33%, #5F0938 100%"
                },
            )
        >
        <div class=move || {
            if alt_style{
                "bg-gradient-to-r from-[#FF78C1] via-[#E2017B] to-[#5F0938] inline-block text-transparent bg-clip-text"
            } else {
                "text-white"
            }
        }>{children()}</div>

        </a>
    }
}

#[component]
pub fn SecondaryHighlightedLinkButton(
    children: Children,
    href: String,
    #[prop(optional)] classes: String,
    #[prop(optional)] alt_style: Signal<bool>,
) -> impl IntoView {
    view! {
        <a
            href=href
            class=move || format!(
                "rounded-full border border-white text-sm font-bold font-kumbh px-5 py-2 {} {}",
                if alt_style.get() {
                    "bg-transparent text-white hover:bg-white/10 active:bg-white/5"
                } else {
                    "bg-white text-black"
                },
                classes,
            )
        >
            {children()}
        </a>
    }
}

#[component]
pub fn SecondaryHighlightedButton(
    children: Children,
    disabled: Signal<bool>,
    alt_style: Signal<bool>,
    classes: String,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    let on_click = move |_| on_click();
    view! {
        <button
            disabled=move || disabled.get()
            on:click=on_click
            class=move ||format!(
                "rounded-full border border-white text-sm font-bold font-kumbh px-5 py-2 {} {}",
                if alt_style.get() {
                    "bg-transparent text-white hover:bg-white/10 active:bg-white/5"
                } else {
                    "bg-white text-black"
                },
                classes,
            )
        >

            {children()}
        </button>
    }
}
