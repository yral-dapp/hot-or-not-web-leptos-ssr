mod items;

use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
fn FaqItem(#[prop(into)] header: String, #[prop(into)] content: String) -> impl IntoView {
    let show = create_rw_signal(false);

    view! {
        <div class="bg-white/10 w-full p-3 flex flex-col gap-1 rounded-md">
            <div
                class="grid grid-cols-2 items-center w-full cursor-pointer"
                on:click=move |_| show.update(|s| *s = !*s)
            >
                <span class="text-lg">{header}</span>
                <div class="text-orange-600 text-lg justify-self-end">
                    <Show when=show fallback=|| view! { <Icon icon=icondata::AiPlusOutlined/> }>
                        <Icon icon=icondata::AiMinusOutlined/>
                    </Show>
                </div>
            </div>
            <Show when=show>
                <div class="text-sm text-white/70">{content.clone()}</div>
            </Show>
        </div>
    }
}

#[component]
fn FaqType<F: FnMut() + 'static>(
    #[prop(into)] name: String,
    #[prop(optional)] init_checked: bool,
    mut onclick: F,
) -> impl IntoView {
    view! {
        <label class="flex flex-col items-center cursor-pointer" on:click=move |_| onclick()>
            <input
                type="radio"
                value=""
                name="faq-selection"
                class="sr-only peer"
                checked=init_checked
            />
            <span class="text-md text-white/50 peer-checked:text-white">{name}</span>
            <div class="p-1 rounded-full bg-orange-600 hidden peer-checked:block"></div>
        </label>
    }
}

#[component]
fn FaqView(
    tab_idx: usize,
    cur_tab: Signal<usize>,
    tab_content: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    view! {
        <Show when=move || {
            tab_idx == cur_tab()
        }>
            {tab_content
                .iter()
                .map(|(header, content)| view! { <FaqItem header=*header content=*content/> })
                .collect_view()}
        </Show>
    }
}

#[component]
fn FaqSwitcher() -> impl IntoView {
    let (cur_tab, set_cur_tab) = create_query_signal::<String>("tab");
    let current_tab = Signal::derive(move || {
        with!(|cur_tab| match cur_tab.as_deref() {
            Some("general") => 0,
            Some("tokens") => 1,
            Some("nfts") => 2,
            _ => 0,
        })
    });

    view! {
        <div class="flex flex-row gap-6 mb-4">
            <FaqType
                name="General"
                onclick=move || set_cur_tab(Some("general".into()))
                init_checked=true
            />
            <FaqType name="Tokens" onclick=move || set_cur_tab(Some("tokens".into()))/>
            <FaqType name="NFTs" onclick=move || set_cur_tab(Some("nfts".into()))/>
        </div>
        <div class="flex flex-col gap-4 w-full">
            <FaqView tab_idx=0 cur_tab=current_tab tab_content=&items::GENERAL_ITEMS/>
            <FaqView tab_idx=1 cur_tab=current_tab tab_content=&items::TOKENS_ITEMS/>
            <FaqView tab_idx=2 cur_tab=current_tab tab_content=&items::NFTS_ITEMS/>
        </div>
    }
}

#[component]
pub fn Faq() -> impl IntoView {
    view! {
        <div class="w-screen min-h-screen px-8 bg-black pt-4 pb-14 text-white flex flex-col items-center">
            <span class="font-bold">FAQs</span>
            <div class="w-full text-lg my-8">Find all your answers here</div>
            <FaqSwitcher/>
        </div>
    }
}
