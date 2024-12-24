use crate::{
    component::{buttons::HighlightedButton, spinner::SpinnerFit},
    state::canisters::authenticated_canisters,
};
use candid::Nat;
use leptos::*;
use yral_canisters_common::{utils::token::TokenMetadata, Canisters};

#[component]
pub fn AirdropPage(meta: TokenMetadata, airdrop_amount: u64) -> impl IntoView {
    let (claimed, set_claimed) = create_signal(false);

    let bg_image = "/img/airdrop/bg.webp";
    let cloud_image = "/img/airdrop/cloud.webp";
    let parachute_image = "/img/airdrop/parachute.webp";

    let (buffer_signal, set_buffer_signal) = create_signal(false);

    let meta_c = meta.clone();
    let meta_c2 = meta.clone();

    let cans_res = authenticated_canisters();
    let airdrop_action = create_action(move |&()| {
        let cans_res = cans_res.clone();
        let token_owner_cans_id = meta.token_owner.clone().unwrap().canister_id;
        async move {
            if claimed.get() && !buffer_signal.get() {
                return Ok(());
            }
            set_buffer_signal.set(true);
            let cans_wire = cans_res.wait_untracked().await?;
            let cans = Canisters::from_wire(cans_wire, expect_context())?;
            let token_owner = cans.individual_user(token_owner_cans_id).await;

            token_owner
                .request_airdrop(
                    meta.root.clone().unwrap(),
                    None,
                    Into::<Nat>::into(airdrop_amount) * 10u64.pow(8),
                    cans.user_canister(),
                )
                .await?;

            token_owner
                .deployed_cdao_canisters()
                .await?
                .into_iter()
                .find(|cdao| Some(cdao.root) == meta.root);

            let user = cans.individual_user(cans.user_canister()).await;
            user.add_token(meta.root.unwrap()).await?;

            set_buffer_signal(false);
            set_claimed(true);
            Ok::<_, ServerFnError>(())
        }
    });

    view! {
        <div
            style="background: radial-gradient(circle, rgba(0,0,0,0) 0%, rgba(0,0,0,0) 75%, rgba(50,0,28,0.5) 100%);"
            class="h-screen w-screen relative items-center justify-center gap-8 text-white font-kumbh flex flex-col overflow-hidden"
        >
            <img
                alt="bg"
                src=bg_image
                class="absolute inset-0 z-[1] fade-in w-full h-full object-cover"
            />

            {move || {
                let meta = meta_c.clone();
                if !claimed.get() {
                    view! {
                        <div class="relative h-[24rem] z-[2]">
                            <div
                                style="--y: 50px"
                                class="flex flex-col items-center justify-center airdrop-parachute"
                            >
                                <img alt="Parachute" src=parachute_image class="h-72 shrink-0" />

                                <div
                                    style="background: radial-gradient(circle, rgb(244 141 199) 0%, rgb(255 255 255) 100%); box-shadow: 0px 0px 3.43px 0px #FFFFFF29;"
                                    class="p-[1px] w-16 h-16 -translate-y-8 rounded-full"
                                >
                                    <img
                                        alt="Airdrop"
                                        src=meta.logo_b64
                                        class="w-full fade-in rounded-full h-full object-cover"
                                    />
                                </div>
                            </div>
                            <img
                                alt="Cloud"
                                src=cloud_image
                                style="--x: -50px"
                                class="w-12 absolute -top-10 left-0 airdrop-cloud"
                            />
                            <img
                                alt="Cloud"
                                src=cloud_image
                                style="--x: 50px"
                                class="w-16 absolute bottom-10 right-10 airdrop-cloud"
                            />
                        </div>
                    }
                } else if claimed.get() {
                    view! {
                        <div class="h-[24rem] w-full flex items-center justify-center z-[2]">
                            <div class="h-[12rem] w-[12rem] relative">
                                <AnimatedTick />
                                <div
                                    style="--duration:1500ms; background: radial-gradient(circle, rgba(27,0,15,1) 0%, rgba(0,0,0,1) 100%); box-shadow: 0px 0px 3.43px 0px #FFFFFF29;"
                                    class="p-[1px] fade-in absolute w-16 h-16 -bottom-4 -right-4 rounded-full"
                                >
                                    <img
                                        alt="Airdrop"
                                        src=meta.logo_b64
                                        class="w-full fade-in rounded-full h-full object-cover"
                                    />
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div class="invisible" /> }
                }
            }}


            <div
                style="--duration:1500ms"
                class="fade-in flex text-xl font-bold z-[2] w-full flex-col gap-4 items-center justify-center px-8"
            >
                {move || {if claimed.get() {
                    let meta = meta_c2.clone();
                    view! {
                        <div class="text-center">
                            {format!("{} {}", airdrop_amount, meta.name)} <br />
                            <span class="font-normal">"added to wallet"</span>
                        </div>
                    }
                } else {
                    view! {
                        <div class="text-center">
                            {format!("{} {} Airdrop received", airdrop_amount, meta.name)}
                        </div>
                    }
                }}}
                <HighlightedButton
                    classes="max-w-96 mx-auto".to_string()
                    alt_style=claimed.into()
                    disabled=buffer_signal.into()
                    on_click=move || {airdrop_action.dispatch(());}
                >
                    {move || {
                        if buffer_signal.get() {
                            view!{<div classes="max-w-90"><SpinnerFit /></div>}.into_view()
                        }else if claimed.get() {
                            view!{<a href="/wallet">"Go to wallet"</a>}.into_view()
                            } else {
                            view!{"Claim Now"}.into_view()
                            }
                        }
                    }
                </HighlightedButton>
            </div>
        </div>
    }
}

#[component]
pub fn AnimatedTick() -> impl IntoView {
    view! {
        <div class="h-full w-full [perspective:800px]">
            <div class="relative h-full w-full scale-110 animate-coin-spin-horizontal rounded-full [transform-style:preserve-3d] before:absolute before:h-full before:w-full before:rounded-full
            before:bg-gradient-to-b before:from-[#FFC6F9] before:via-[#C01271] before:to-[#990D55] before:[transform-style:preserve-3d] before:[transform:translateZ(1px)]">
                <div class="absolute flex h-full w-full items-center justify-center rounded-full text-center [transform:translateZ(2rem)] p-12
                bg-gradient-to-br from-[#C01272] to-[#FF48B2]">
                    <div class="relative">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            xmlns:xlink="http://www.w3.org/1999/xlink"
                            class="h-full w-full text-current [transform-style:preserve-3d] [transform:translateZ(10px)]"
                            viewBox="0 -3 32 32"
                            version="1.1"
                        >
                            <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                                <g
                                    transform="translate(-518.000000, -1039.000000)"
                                    fill="currentColor"
                                >
                                    <path d="M548.783,1040.2 C547.188,1038.57 544.603,1038.57 543.008,1040.2 L528.569,1054.92 L524.96,1051.24 C523.365,1049.62 520.779,1049.62 519.185,1051.24 C517.59,1052.87 517.59,1055.51 519.185,1057.13 L525.682,1063.76 C527.277,1065.39 529.862,1065.39 531.457,1063.76 L548.783,1046.09 C550.378,1044.46 550.378,1041.82 548.783,1040.2"></path>
                                </g>
                            </g>
                        </svg>
                    </div>
                </div>
            </div>
        </div>
    }
}
