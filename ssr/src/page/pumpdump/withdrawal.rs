use candid::{Nat, Principal};
use futures::TryFutureExt;
use http::StatusCode;
use leptos::{
    component, create_action, create_effect, create_rw_signal, event_target_value, expect_context,
    view, IntoView, ServerFnError, SignalSet, Suspense,
};
use leptos_router::use_navigate;
use yral_canisters_common::{utils::token::balance::TokenBalance, Canisters};
use yral_pump_n_dump_common::rest::{BalanceInfoResponse, ClaimReq};

use crate::{
    component::{
        back_btn::BackButton,
        icons::{information_icon::Information, notification_icon::NotificationIcon},
        title::Title,
        tooltip::Tooltip,
    },
    consts::PUMP_AND_DUMP_WORKER_URL,
    format_cents,
    state::canisters::authenticated_canisters,
    try_or_redirect_opt,
};

pub mod result;

type NetEarnings = Nat;

/// Details for withdrawal functionality
type Details = (BalanceInfoResponse, NetEarnings);

async fn load_withdrawal_details(user_canister: Principal) -> Result<Details, String> {
    let balance_info = PUMP_AND_DUMP_WORKER_URL
        .join(&format!("/balance/{user_canister}"))
        .expect("Url to be valid");

    let net_earnings = PUMP_AND_DUMP_WORKER_URL
        .join(&format!("/earnings/{user_canister}"))
        .expect("Url to be valid");

    let balance_info: BalanceInfoResponse = reqwest::get(balance_info)
        .await
        .map_err(|_| "failed to load balance".to_string())?
        .json()
        .await
        .map_err(|_| "failed to read response body".to_string())?;

    let net_earnings: Nat = reqwest::get(net_earnings)
        .await
        .map_err(|err| format!("Coulnd't load net earnings: {err}"))?
        .text()
        .await
        .map_err(|err| format!("Couldn't read response for net earnings: {err}"))?
        .parse()
        .map_err(|err| format!("Couldn't parse net earnings from response: {err}"))?;

    Ok((balance_info, net_earnings))
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div id="back-nav" class="flex flex-col items-center w-full gap-20 pb-16">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/" />
                    <span class="font-bold text-2xl">Withdraw</span>
                    <a href="/wallet/notifications" disabled=true class="text-xl font-semibold">
                        <NotificationIcon show_dot=false classes="w-8 h-8 text-neutral-600".to_string() />
                    </a>
                </div>
            </Title>
        </div>
    }
}

#[component]
fn BalanceDisplay(#[prop(into)] balance: Nat, #[prop(into)] withdrawable: Nat) -> impl IntoView {
    view! {
        <div id="total-balance" class="self-center flex flex-col items-center gap-1">
            <span class="text-neutral-400 text-sm">Total Cent balance</span>
            <div class="flex items-center gap-3 min-h-14 py-0.5">
                <img class="size-9" src="/img/cents.png" alt="cents icon" />
                <span class="font-bold text-4xl">{format_cents!(balance)}</span>
            </div>
        </div>
        <div id="breakdown" class="flex justify-between py-2.5 px-3 bg-neutral-900 w-full gap-8 mt-5 rounded-lg">
            <div class="flex gap-2 items-center">
                <span class="text-xs">
                    Cents you can withdraw
                </span>
                <Tooltip icon=Information title="Withdrawal Tokens" description="Only cents earned above your airdrop amount can be withdrawn." />
            </div>
            <span class="text-lg font-semibold">{format_cents!(withdrawable)}</span>
        </div>
    }
}

#[component]
pub fn PndWithdrawal() -> impl IntoView {
    let auth_wire = authenticated_canisters();
    let details_res = auth_wire.derive(
        move || (),
        move |cans_wire, _| async move {
            let cans_wire = cans_wire?;
            load_withdrawal_details(cans_wire.user_canister)
                .map_err(ServerFnError::new)
                .await
        },
    );
    let cents = create_rw_signal(0);
    let dolrs = move || Nat::from(cents()) * 1e6 as usize;
    let formated_dolrs = move || {
        format!(
            "{}DOLR",
            TokenBalance::new(dolrs(), 8).humanize_float_truncate_to_dp(2)
        )
    };
    let on_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        let value = value.parse::<usize>().ok();
        let Some(value) = value else { return };

        cents.set(value);
    };

    let auth_wire = authenticated_canisters();
    let send_claim = create_action(move |&()| {
        let auth_wire = auth_wire.clone();
        async move {
            let auth_wire = auth_wire
                .wait_untracked()
                .await
                .map_err(ServerFnError::new)?;

            let cans = Canisters::from_wire(auth_wire.clone(), expect_context())
                .map_err(ServerFnError::new)?;

            let req = ClaimReq::new(cans.identity(), dolrs()).map_err(ServerFnError::new)?;
            let claim_url = PUMP_AND_DUMP_WORKER_URL
                .join("/claim_gdollr")
                .expect("Url to be valid");
            let client = reqwest::Client::new();
            let res = client
                .post(claim_url)
                .json(&req)
                .send()
                .await
                .map_err(ServerFnError::new)?;

            if res.status() != StatusCode::OK {
                return Err(ServerFnError::new("Request failed"));
            }

            Ok::<(), ServerFnError>(())
        }
    });
    let is_claiming = send_claim.pending();
    let claim_res = send_claim.value();
    create_effect(move |_| {
        if let Some(res) = claim_res() {
            let nav = use_navigate();
            match res {
                Ok(_) => {
                    nav(
                        &format!("/pnd/withdraw/success?cents={}", cents()),
                        Default::default(),
                    );
                }
                Err(err) => {
                    nav(
                        &format!("/pnd/withdraw/failure?cents={}&err={err}", cents()),
                        Default::default(),
                    );
                }
            }
        }
    });
    view! {
        <div class="min-h-screen w-full flex flex-col text-white pt-2 pb-12 bg-black items-center overflow-x-hidden">
            <Header />
            <div class="w-full">
                <div class="flex flex-col items-center justify-center max-w-md mx-auto px-4 mt-4 pb-6">
                    <Suspense>
                    {move || {
                        let (balance_info, _) = try_or_redirect_opt!(details_res()?);
                        Some(view! {
                            <BalanceDisplay balance=balance_info.balance withdrawable=balance_info.withdrawable />
                        })
                    }}
                    </Suspense>
                    <div class="flex flex-col gap-5 mt-8 w-full">
                        <span class="text-sm">Choose how much to redeem:</span>
                        <div id="input-card" class="rounded-lg bg-neutral-900 p-3 flex flex-col gap-8">
                            <div class="flex flex-col gap-3">
                                <div class="flex justify-between">
                                    <div class="flex gap-2 items-center">
                                        <span>You withdraw</span>
                                        <Tooltip icon=Information title="Withdrawal Tokens" description="Only cents earned above your airdrop amount can be withdrawn." />
                                    </div>
                                    <input disabled=is_claiming on:input=on_input type="text" inputmode="decimal" class="bg-neutral-800 h-10 w-32 rounded focus:outline focus:outline-1 focus:outline-[#E2017B] text-right px-4 text-lg" />
                                </div>
                                <div class="flex justify-between">
                                    <div class="flex gap-2 items-center">
                                        <span>You get</span>
                                    </div>
                                    <input disabled type="text" inputmode="decimal" class="bg-neutral-800 h-10 w-32 rounded focus:outline focus:outline-1 focus:outline-[#E2017B] text-right px-4 text-lg text-neutral-400" value=formated_dolrs />
                                </div>
                            </div>
                            <Suspense fallback=|| view! {
                                <button
                                    disabled
                                    class="rounded-lg px-5 py-2 text-sm text-center font-bold bg-brand-gradient-disabled"
                                >Please Wait</button>
                            }>
                            {move || {
                                let (BalanceInfoResponse { withdrawable, .. }, _) = try_or_redirect_opt!(details_res()?);
                                let can_withdraw = withdrawable > cents();
                                let message = match (can_withdraw, is_claiming()) {
                                    (false, _) if cents() > 0 => "Not enough winnings",
                                    (false, _) => "Enter Amount",
                                    (_, true) => "Claiming...",
                                    (_, _) => "Withdraw Now!"
                                };
                                Some(view! {
                                    <button
                                        disabled=!can_withdraw || is_claiming()
                                        class=("bg-brand-gradient", can_withdraw)
                                        class=("bg-brand-gradient-disabled", !can_withdraw)
                                        class="rounded-lg px-5 py-2 text-sm text-center font-bold"
                                        on:click=move |_ev| send_claim.dispatch(())
                                    >{message}</button>
                                })
                            }}
                            </Suspense>
                        </div>
                        <span class="text-sm">1 Cent = 0.01 DOLR</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
