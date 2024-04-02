mod history;

use candid::Principal;
use ic_agent::Identity;
use leptos::*;
use leptos_icons::*;
use leptos_router::create_query_signal;
use leptos_use::use_window;
use serde_json::json;

use crate::state::history::HistoryCtx;
use crate::utils::event_streaming::send_event;
use crate::{
    component::{back_btn::BackButton, title::Title},
    state::{
        auth::account_connected_reader,
        canisters::{authenticated_canisters, AuthProfileCanisterResource},
    },
    try_or_redirect_opt,
    utils::web::copy_to_clipboard,
};
use history::HistoryView;

#[component]
fn WorkButton(#[prop(into)] text: String, #[prop(into)] head: String) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3">
            <div class="grid place-items-center rounded-sm font-semibold">{head}</div>
            <span class="text-xs md:text-sm whitespace-pre-line">{text}</span>
        </div>
    }
}

#[component]
fn ReferLoaded(user_principal: Principal) -> impl IntoView {
    let refer_code = user_principal.to_text();
    let window = use_window();
    let refer_link = window
        .as_ref()
        .and_then(|w| {
            let origin = w.location().origin().ok()?;
            Some(format!(
                "{}/?user_refer={}",
                origin,
                user_principal.to_text()
            ))
        })
        .unwrap_or_default();
    let (logged_in, _) = account_connected_reader();
    let profile_and_canister_details: AuthProfileCanisterResource = expect_context();

    let click_copy = move || {
        let _ = copy_to_clipboard(&refer_link);

        let user_id = move || {
            profile_and_canister_details()
                .flatten()
                .map(|(q, _)| q.principal)
        };
        let display_name = move || {
            profile_and_canister_details()
                .flatten()
                .map(|(q, _)| q.display_name)
        };
        let canister_id = move || profile_and_canister_details().flatten().map(|(_, q)| q);
        let history_ctx: HistoryCtx = expect_context();
        let prev_site = history_ctx.prev_url();

        // refer_share_link - analytics
        create_effect(move |_| {
            send_event(
                "refer_share_link",
                &json!({
                    "user_id":user_id(),
                    "is_loggedIn": logged_in.get_untracked(),
                    "display_name": display_name(),
                    "canister_id": canister_id(),
                    "refer_location": prev_site,
                }),
            );
        });
    };

    view! {
        <div class="flex items-center w-fit rounded-full border-dashed border-2 p-3 gap-2 border-primary-500">
            <span class="text-md lg:text-lg text-ellipsis line-clamp-1">{refer_code}</span>
            <button on:click=move |_| click_copy()>
                <Icon class="text-xl" icon=icondata::FaCopyRegular/>
            </button>
        </div>
    }
}

#[component]
fn ReferLoading() -> impl IntoView {
    view! {
        <div class="flex border-dashed w-full md:w-2/12 p-1 h-10 md:h-12 border-2 border-primary-500 rounded-full">
            <span class="bg-white/30 w-full h-full animate-pulse rounded-full "></span>
        </div>
    }
}

#[component]
fn ReferCode() -> impl IntoView {
    let canisters = authenticated_canisters();

    view! {
        <Suspense fallback=ReferLoading>
            {move || {
                canisters()
                    .and_then(|canisters| {
                        let canisters = try_or_redirect_opt!(canisters)?;
                        let user_principal = canisters.identity().sender().unwrap();
                        Some(view! { <ReferLoaded user_principal/> })
                    })
                    .unwrap_or_else(|| {
                        view! { <ReferLoading/> }
                    })
            }}

        </Suspense>
    }
}

#[component]
fn ReferView() -> impl IntoView {
    let (logged_in, _) = account_connected_reader();

    let profile_and_canister_details: AuthProfileCanisterResource = expect_context();
    let user_id = move || {
        profile_and_canister_details()
            .flatten()
            .map(|(q, _)| q.principal)
    };
    let display_name = move || {
        profile_and_canister_details()
            .flatten()
            .map(|(q, _)| q.display_name)
    };
    let canister_id = move || profile_and_canister_details().flatten().map(|(_, q)| q);
    let history_ctx: HistoryCtx = expect_context();
    let prev_site = history_ctx.prev_url();

    // refer - analytics
    create_effect(move |_| {
        send_event(
            "refer",
            &json!({
                "user_id":user_id(),
                "is_loggedIn": logged_in.get_untracked(),
                "display_name": display_name(),
                "canister_id": canister_id(),
                "refer_location": prev_site,
            }),
        );
    });

    view! {
        <div class="flex flex-col w-full h-full items-center text-white gap-10">
            <img class="shrink-0 h-40 select-none" src="/img/coins-stash.webp"/>
            <div class="flex flex-col w-full items-center gap-4 text-center">
                <span class="font-bold text-2xl">Invite & Win upto <br/> 500 Coyns</span>
            </div>
            <div class="flex flex-col w-full gap-2 px-4 text-white items-center">
                <span class="uppercase text-sm md:text-md">Referral Link</span>
                // <Show when=logged_in fallback=|| view! { <ConnectLogin/> }>
                    <ReferCode/>
                // </Show>
            </div>
            <div class="flex flex-col w-full items-center gap-8 mt-4">
                <span class="font-xl font-semibold">HOW IT WORKS?</span>
                <div class="flex flex-row gap-8 text-center">
                    <WorkButton
                        text="Share your link
                        with a friend"
                        head="STEP 1"
                    />
                    <WorkButton
                        text="Your friend uses
                        the shared link
                        to login"
                        head="STEP 2"
                    />
                    <WorkButton
                        text="You both win
                        500 Coyns each"
                        head="STEP 3"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn TabSelector(
    tab_idx: i32,
    text: String,
    tab_str: String,
    current_tab: Memo<i32>,
    set_cur_tab: SignalSetter<Option<String>>,
) -> impl IntoView {
    let button_class = move || {
        if tab_idx == current_tab() {
            "text-white font-bold"
        } else {
            "text-white/50 font-bold"
        }
    };
    let selector_class = move || {
        if tab_idx == current_tab() {
            "bg-primary-500 w-2 h-2 rounded-full"
        } else {
            "bg-transparent w-2 h-2 rounded-full"
        }
    };

    view! {
        <div class="flex w-full flex-col items-center gap-y-2">
            <button class=button_class on:click=move |_| set_cur_tab(Some(tab_str.clone()))>
                {text}
            </button>
            <div class=selector_class></div>
        </div>
    }
}

#[component]
fn ListSwitcher() -> impl IntoView {
    let (cur_tab, set_cur_tab) = create_query_signal::<String>("tab");
    let current_tab = create_memo(move |_| {
        with!(|cur_tab| match cur_tab.as_deref() {
            Some("how-to") => 0,
            Some("history") => 1,
            _ => 0,
        })
    });

    view! {
        <div class="flex flex-row w-full text-md md:text-lg lg:text-xl text-center">
            <TabSelector
                text="How to earn".into()
                tab_idx=0
                tab_str="how-to".to_string()
                current_tab
                set_cur_tab=set_cur_tab
            />
            <TabSelector
                text="History".into()
                tab_idx=1
                tab_str="history".to_string()
                current_tab
                set_cur_tab=set_cur_tab
            />
        </div>
        <div class="flex flex-row justify-center">
            <Show when=move || current_tab() == 0 fallback=HistoryView>
                <ReferView/>
            </Show>
        </div>
    }
}

#[component]
pub fn ReferEarn() -> impl IntoView {
    let (logged_in, _) = account_connected_reader();

    view! {
        <div class="flex flex-col items-center min-w-dvw min-h-dvh bg-black pt-2 pb-12 gap-6">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/menu".to_string()/>
                    <span class="text-lg font-bold text-white">Refer & Earn</span>
                    <div></div>
                </div>
            </Title>
            <div class="px-8 w-full sm:w-7/12">
                <Show when=logged_in fallback=ReferView>
                    <ListSwitcher/>
                </Show>
            </div>
        </div>
    }
}
