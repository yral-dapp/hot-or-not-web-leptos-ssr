use candid::Principal;
use leptos::{
    component, create_action, create_effect, create_rw_signal, expect_context, html::Div, view,
    For, IntoView, NodeRef, RwSignal, Show, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate,
    SignalUpdateUntracked,
};
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use yral_canisters_client::individual_user_template::{
    GameDirection, IndividualUserTemplate, ParticipatedGameInfo,
};
use yral_canisters_common::{utils::profile::ProfileDetails, Canisters};

use crate::{
    component::{back_btn::BackButton, skeleton::Skeleton, title::TitleText},
    page::pumpdump::{convert_e8s_to_cents, GameResult},
    state::canisters::authenticated_canisters,
};

use super::GameState;

#[derive(Debug, Clone)]
pub(super) struct ProfileData {
    pub(super) user: ProfileDetails,
}

impl ProfileData {
    pub(super) async fn load(
        user: ProfileDetails,
        _ind_user: IndividualUserTemplate<'_>,
    ) -> Result<Self, String> {
        Ok(Self { user })
    }
}

type ProfileDataSignal = RwSignal<Option<ProfileData>>;

#[derive(Debug, Clone)]
struct GameplayHistoryItem {
    logo: String,
    root: Principal,
    owner_principal: Principal,
    owner_pfp: String,
    state: GameState,
}

type GameplayHistory = Vec<GameplayHistoryItem>;

fn compute_result(info: ParticipatedGameInfo) -> GameResult {
    let user_direction = match info.pumps.cmp(&info.dumps) {
        std::cmp::Ordering::Greater => GameDirection::Pump,
        std::cmp::Ordering::Less => GameDirection::Dump,
        std::cmp::Ordering::Equal => return GameResult::Win { amount: 0 },
    };

    // TODO: make game direction comparable
    let m = |d: GameDirection| match d {
        GameDirection::Pump => 0,
        GameDirection::Dump => 1,
    };

    if m(user_direction) == m(info.game_direction) {
        GameResult::Win {
            amount: convert_e8s_to_cents(info.reward),
        }
    } else {
        GameResult::Loss {
            amount: info.pumps as u128 + info.dumps as u128,
        }
    }
}

// TODO: switch to using in-house `InfiniteScroller` for 0.7 migration
async fn load_history(cans: Canisters<true>, page: u64) -> Result<(GameplayHistory, bool), String> {
    // to test without playing games
    // #[cfg(any(feature = "local-lib", feature = "local-bin"))]
    // {
    //     let history = vec![GameplayHistoryItem {
    //         logo: "https://picsum.photos/200".into(),
    //         owner_pfp: "https://picsum.photos/200?rand=1".into(),
    //         owner_principal: Principal::anonymous(),
    //         root: Principal::anonymous(),
    //         state: GameState::ResultDeclared(GameResult::Win { amount: 100 }),
    //     }];

    //     return Ok((history, true));
    // }

    use crate::utils::token::icpump::IcpumpTokenInfo;
    use yral_canisters_common::utils::token::RootType;

    let limit = 25;
    let start_index = page * limit;
    let items = cans
        .individual_user(cans.user_canister())
        .await
        .played_game_info_with_pagination_cursor(start_index, limit)
        .await
        .map_err(|err| format!("Couldn't load gameplay history: {err}"))?;
    let items = match items {
        yral_canisters_client::individual_user_template::Result21::Ok(res) => res,
        yral_canisters_client::individual_user_template::Result21::Err(err) => {
            return Err(format!("Couldn't load played games: {err}"));
        }
    };
    let had_items = !items.is_empty();

    let mut processed_items = Vec::with_capacity(items.len());
    for item in items {
        let meta = cans
            .token_metadata_by_root_type(
                &IcpumpTokenInfo,
                Some(cans.user_canister()),
                RootType::Other(item.token_root),
            )
            .await
            .ok()
            .flatten()
            .expect("backend to return a token that exists");

        let pfp = cans
            .individual_user(
                meta.token_owner
                    .as_ref()
                    .expect("owner to exist")
                    .canister_id,
            )
            .await
            .get_profile_details()
            .await
            .map(Into::<ProfileDetails>::into)
            .map_err(|err| format!("Couldn't load owner profile details: {err}"))?
            .profile_pic_or_random();

        processed_items.push(GameplayHistoryItem {
            logo: meta.logo_b64,
            owner_pfp: pfp,
            owner_principal: meta
                .token_owner
                .expect("owner to exist if backend returns it")
                .principal_id,
            root: item.token_root,
            state: GameState::ResultDeclared(compute_result(item)),
        })
    }

    Ok((processed_items, had_items))
}

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
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GameplayHistorySkeleton() -> impl IntoView {
    view! {
        <Skeleton class="text-neutral-800 [--shimmer:#363636] max-w-32 max-h-40 w-32 h-40 rounded-md" />
    }
}

#[component]
fn GameplayHistoryCard(#[prop(into)] details: GameplayHistoryItem) -> impl IntoView {
    let state = details.state;
    let href = {
        let root = details.root;
        let (state_label, amount) = match state {
            GameState::ResultDeclared(result) => match result {
                GameResult::Win { amount } => ("win", amount),
                GameResult::Loss { amount } => ("loss", amount),
            },
            _ => unreachable!("gameplay history only includes games with result declared"),
        };

        format!("/?root={root}&state={state_label}&amount={amount}")
    };
    view! {
        <a href=href>
            <div class="rounded-md overflow-hidden relative max-w-32 max-h-40 w-32 h-40">
                <div class="absolute z-1 inset-x-0 h-1/3 bg-gradient-to-b from-black/50 to-transparent"></div>
                <div class="absolute z-[2] flex top-2 items-center gap-1 px-2">
                    <img src=details.owner_pfp alt="Profile name" class="w-4 h-4 shrink-0 object-cover rounded-full" />
                    <span class="text-xs font-medium line-clamp-1">{details.owner_principal.to_string()}</span>
                </div>
                <img src=details.logo class="w-full bg-white/5 h-28 object-cover" alt="Coin title" />
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
                        class=(["bg-primary-800", "text-white"], state.has_won())
                        class=(["text-neutral-400", "bg-[#212121]"], state.has_lost())
                    >
                        {if state.has_lost() { "You Lost" } else { "You Won" }}
                    </div>
                    <div
                        class="text-xs font-semibold py-1 text-center"
                        class=(["bg-primary-600", "text-white"], state.has_won())
                        class=(["bg-neutral-600", "text-neutral-400"], state.has_lost())
                    >
                        {state.winnings().or(state.lossings())} Cents
                    </div>
                </Show>
            </div>
        </a>
    }
}

#[component]
pub fn PndProfilePage() -> impl IntoView {
    let profile_data: ProfileDataSignal = create_rw_signal(None);
    // TODO: write the create_effect and action combo for profile data

    let gameplay_history: GameplayHistorySignal = create_rw_signal(Default::default());

    let auth_cans = authenticated_canisters();
    let auth_cans_for_profile = auth_cans.clone();
    let load_profile_data = create_action(move |&()| {
        let value = auth_cans_for_profile.clone();
        async move {
            let cans_wire = value.wait_untracked().await.map_err(|e| e.to_string())?;

            let user = cans_wire.profile_details.clone();
            let canisters = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|e| e.to_string())?;

            let ind_user = canisters.individual_user(canisters.user_canister()).await;

            // TODO: send telemetry or something for these errors
            profile_data.set(Some(ProfileData::load(user, ind_user).await?));

            Ok::<_, String>(())
        }
    });

    create_effect(move |_| {
        if profile_data.get_untracked().is_none() {
            load_profile_data.dispatch(());
        }
    });

    let auth_can_for_history = auth_cans.clone();
    let page = create_rw_signal(0);
    let should_load_more = create_rw_signal(true);
    let load_gameplay_history = create_action(move |&page| {
        let cans_wire_res = auth_can_for_history.clone();
        async move {
            // since we are starting a load job, no more load jobs should be start
            should_load_more.set(false);
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|_| "Couldn't get cans_wire")?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            let (processed_items, had_items) = load_history(cans, page).await?;
            gameplay_history.update(|list| {
                list.extend(processed_items);
            });

            if had_items {
                // since there were tokens loaded
                // assume we have more tokens to load
                // so, allow token loading
                should_load_more.set(true)
            }

            Ok::<_, String>(())
        }
    });

    let scroll_container = NodeRef::<Div>::new();
    let is_loading = use_infinite_scroll_with_options(
        scroll_container,
        move |_| async move {
            if !should_load_more.get() {
                return;
            }
            load_gameplay_history.dispatch(page.get_untracked());
            page.update_untracked(|v| {
                *v += 1;
            });
        },
        UseInfiniteScrollOptions::default()
            .distance(400f64)
            .interval(2000f64),
    );

    view! {
        <div class="min-h-screen w-full flex flex-col text-white pt-2 pb-12 bg-black items-center">
            <div id="back-nav" class="flex flex-col items-center w-full gap-20 pb-16">
                <TitleText justify_center=false>
                    <div class="flex flex-row justify-between">
                        <BackButton fallback="/" />
                        <span class="font-bold text-2xl">Profile</span>
                        <div></div>
                    </div>
                </TitleText>
            </div>
            <Show when=move || profile_data.get().is_some()>
                <ProfileDataSection profile_data=profile_data.get().unwrap() />
            </Show>
            <div class="w-11/12 flex justify-center">
                <div ref=scroll_container class="flex flex-wrap gap-4 justify-center pt-8 pb-16">
                    <For each=move || gameplay_history.get().into_iter().enumerate() key=|(idx, _)| *idx let:item>
                        <GameplayHistoryCard details=item.1 />
                    </For>
                    <Show when=move || is_loading() && should_load_more.get_untracked()>
                        <For each=move || 0..6 key=|&idx| idx let:_>
                            <GameplayHistorySkeleton />
                        </For>
                    </Show>
                </div>
            </div>
        </div>
    }
}
