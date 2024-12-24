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
                "w-full px-5 py-3 rounded-lg disabled:text-white/50 flex items-center transition-all justify-center gap-8 font-kumbh font-bold {} {}",
                if alt_style{
                    "text-primary-600"
                } else {
                    "text-white"
                },
                classes,
            )
            style=format!(
                "background: linear-gradient(73deg, {} );",
                if disabled {
                    "#DE98BE 0%, #E761A9 33%, #7B5369 100%"
                } else if alt_style {
                    "#FFF 0%, #FFF 1000%"
                } else {
                    "#DA539C 0%, #E2017B 33%, #5F0938 100%"
                },
            )
        >
            {children()}
        </button>
    }
}

#[component]
pub fn HighlightedLinkButton(
    children: Children,
    href: String,
    #[prop(optional)] classes: String,
    #[prop(optional)] alt_style: Signal<bool>,
    #[prop(optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <a
            href=href
            disabled=move || disabled.get()
            class=move ||format!(
                "w-full px-5 py-3 rounded-lg {} disabled:text-white/50 flex items-center transition-all justify-center gap-8 font-kumbh font-bold {}",
                if alt_style.get() {
                    "text-primary-600"
                } else {
                    "text-white"
                },
                classes,
            )
            style=move || format!(
                "background: linear-gradient(73deg, {} );",
                if disabled.get() {
                    "#DE98BE 0%, #E761A9 33%, #7B5369 100%"
                } else if alt_style.get() {
                    "#FFF 0%, #FFF 1000%"
                } else {
                    "#DA539C 0%, #E2017B 33%, #5F0938 100%"
                },
            )
        >
            {children()}
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
