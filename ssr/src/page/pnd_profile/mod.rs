use candid::Principal;
use leptos::{
    component, create_action, create_effect, create_rw_signal, view, For, IntoView, RwSignal, Show,
    SignalGet, SignalGetUntracked, SignalSet,
};
use yral_canisters_common::utils::{profile::ProfileDetails, token::TokenOwner};

use crate::{
    component::{back_btn::BackButton, title::Title},
    state::canisters::authenticated_canisters,
    utils::token::icpump::TokenListItem,
};

use super::icpump::{pumpndump::GameState, ProcessedTokenListResponse};

#[derive(Debug, Clone)]
struct ProfileData {
    user: ProfileDetails,
    earnings: u128,
    pumps: u64,
    dumps: u64,
}

type ProfileDataSignal = RwSignal<Option<ProfileData>>;

#[derive(Debug, Clone)]
struct GameplayHistoryItem {
    token: ProcessedTokenListResponse,
    state: GameState,
}

type GameplayHistory = Vec<GameplayHistoryItem>;

type GameplayHistorySignal = RwSignal<GameplayHistory>;

#[component]
fn Stat(stat: u64, #[prop(into)] info: String) -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col items-center text-white space-y-0.5">
            <span class="font-bold text-xl text-neutral-50">{stat}</span>
            <span class="text-md text-neutral-400">{info}</span>
        </div>
    }
}

#[component]
fn ProfileDataSection(#[prop(into)] profile_data: ProfileData) -> impl IntoView {
    let username_or_principal = profile_data.user.username_or_principal();
    let profile_pic = profile_data.user.profile_pic_or_random();
    let display_name = profile_data.user.display_name_or_fallback();

    view! {
        <div class="grid grid-cols-1 gap-5 justify-normal justify-items-center w-full">
            <div class="flex flex-row w-11/12 justify-center">
                <div class="flex flex-col justify-center items-center gap-4">
                    <img
                        class="h-24 w-24 rounded-full"
                        alt=username_or_principal.clone()
                        src=profile_pic
                    />
                    <div class="flex flex-col text-center items-center gap-4">
                        <span
                            class="text-md text-white font-bold w-full"
                        >
                            {display_name}
                        </span>
                        <div
                            class="bg-neutral-900 shrink-0 rounded-full relative w-56 h-11 flex items-center justify-center gap-1"
                        >
                            <span class="text-xs text-neutral-400">Earnings:</span>
                            <img src="/img/gdolr.png" alt="coin" class="w-5 h-5" />
                            <span class="font-bold">{profile_data.earnings} gDOLR</span>
                        </div>
                    </div>
                </div>
            </div>
            <div class="flex justify-around text-center rounded-full divide-x-2 divide-white/20 bg-white/10 py-3 px-14 w-11/12">
                <Stat stat=profile_data.pumps info="PUMPS" />
                <Stat stat=profile_data.dumps info="DUMPS" />
            </div>
        </div>
    }
}

#[component]
fn GameplayHistoryCard(#[prop(into)] details: GameplayHistoryItem) -> impl IntoView {
    let state = details.state;
    view! {
        <div class="rounded-md overflow-hidden relative w-32 h-40">
            <div class="absolute z-1 inset-x-0 h-1/3 bg-gradient-to-b from-black/50 to-transparent"></div>
            <div class="absolute z-[2] flex top-2 items-center gap-1 px-2">
                <img src="/img/gdolr.png" alt="Profile name" class="w-4 h-4 shrink-0 object-cover rounded-full" />
                <span class="text-xs font-medium line-clamp-1">{details.token.token_owner.as_ref().unwrap().principal_id.to_string()}</span>
            </div>
            <img src=details.token.token_details.logo class="w-full bg-white/5 h-28 object-cover" alt="Coin title" />
            <Show
                when=move || !state.is_running()
                fallback=move || view! {
                    <div
                        style="background: linear-gradient(248.46deg, rgba(61, 142, 255, 0.4) 16.67%, rgba(57, 0, 89, 0.4) 52.62%, rgba(226, 1, 123, 0.4) 87.36%);"
                        class="text-xs font-semibold justify-center py-4 flex items-center text-[#FAFAFA]"
                    >
                        Pending
                    </div>
                }
            >
                <div class="text-xs font-semibold py-1 text-center"
                    class=("bg-[#A00157] text-white", state.has_won())
                    class=("text-[#A3A3A3] bg-[#212121]", state.has_lost())
                >
                    {if state.has_lost() { "You Lost" } else { "You Won" }}
                </div>
                <div
                    class="text-xs font-semibold py-1 text-center"
                    class=("bg-[#E2017B] text-white", state.has_won())
                    class=("bg-[#525252] text-[#A3A3A3]", state.has_lost())
                >
                    {state.winnings().or(Some(0))} gDOLR
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn PndProfilePage() -> impl IntoView {
    let profile_data: ProfileDataSignal = create_rw_signal(None);
    // TODO: write the create_effect and action combo for profile data

    let gameplay_history: GameplayHistorySignal = create_rw_signal(vec![
        GameplayHistoryItem {
            token: ProcessedTokenListResponse {
                token_details: TokenListItem {
                    user_id: "user_id".into(),
                    name: "Name".into(),
                    token_name: "MY Dolr".into(),
                    token_symbol: "DOLR".into(),
                    logo: "https://picsum.photos/200".into(),
                    description: "i dont care".into(),
                    created_at: "10hrs".into(),
                    formatted_created_at: "10hrs ago".into(),
                    link: "https://example.com/token".into(),
                    is_nsfw: false
                },
                root: Principal::anonymous(),
                token_owner: Some(TokenOwner {
                    principal_id: Principal::anonymous(),
                    canister_id: Principal::anonymous()
                }),
                is_airdrop_claimed: false
            },
            state: GameState::Playing
        };
        20
    ]);

    let auth_cans = authenticated_canisters();
    let load_profile_data = create_action(move |&()| {
        let value = auth_cans.clone();
        async move {
            let cans_wire = value.wait_untracked().await.map_err(|e| e.to_string())?;

            let user = cans_wire.profile_details.clone();

            // TODO: load pnd related details

            profile_data.set(Some(ProfileData {
                user,
                earnings: 0,
                pumps: 0,
                dumps: 0,
            }));

            Ok::<_, String>(())
        }
    });

    create_effect(move |_| {
        if profile_data.get_untracked().is_none() {
            load_profile_data.dispatch(());
        }
    });

    view! {
        <div class="min-h-screen w-full flex flex-col text-white pt-2 pb-12 bg-black items-center">
            <div id="back-nav" class="flex flex-col items-center w-full gap-20 pb-16">
                <Title justify_center=false>
                    <div class="flex flex-row justify-between">
                        <BackButton fallback="/" />
                        <span class="font-bold text-2xl">Profile</span>
                        <div></div>
                    </div>
                </Title>
            </div>
            <Show when=move || profile_data.get().is_some()>
                <ProfileDataSection profile_data=profile_data.get().unwrap() />
            </Show>
            <div class="w-11/12 flex justify-center">
                <div class="grid grid-cols-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4 pt-8 pb-16">
                    <For each=move || gameplay_history.get().into_iter().enumerate() key=|(idx, _)| *idx let:item>
                        <GameplayHistoryCard details=item.1 />
                    </For>
                </div>
            </div>
        </div>
    }
}
