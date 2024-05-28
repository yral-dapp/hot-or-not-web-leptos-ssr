use ic_agent::Identity;
use leptos::*;

use crate::component::canisters_prov::AuthCansProvider;
use crate::component::connect::ConnectLogin;
use crate::{
    component::{
        back_btn::BackButton,
        dashbox::{DashboxLoaded, DashboxLoading},
        title::Title,
    },
    state::auth::account_connected_reader,
};

#[component]
fn PrincipalInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=DashboxLoading let:cans>
            <span class="uppercase text-sm md:text-md pb-5">COPY PRINICIPAL ID</span>
            <DashboxLoaded text=cans.identity().sender().unwrap().to_text()/>
            <div class="pt-5 pb-10">
                <a href="https://hotornot.wtf/migrate" target="_blank">
                <span class="text-md underline decoration-pink-500 text-pink-500">Visit HotorNot to complete transfer</span>
                </a>
            </div>
        </AuthCansProvider>
    }
}

#[component]
fn PrincipalInfoView() -> impl IntoView {
    let (logged_in, _) = account_connected_reader();

    view! {
        <div class="flex flex-col w-full h-full items-center text-white gap-10">
            <img class="shrink-0 h-40 select-none" src="/img/account-transfer.svg"/>
            <div class="flex flex-col w-full items-center gap-4 text-center">
                <span class="text-md">Transfer your Videos and COYN tokens from your old HotorNot account to your Yral account. We are phasing out HotorNot, so transfer your account before time runs out. <br/> . . . . . . . . . . . </span>
            </div>
            <div class="flex flex-col w-full gap-2 px-4 text-white items-center">
                <Show when=logged_in fallback=|| view! { <ConnectLogin cta_location="refer"/> }>
                    <PrincipalInfo/>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn AccountTransfer() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center min-w-dvw min-h-dvh bg-black pt-2 pb-12 gap-6">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/menu".to_string()/>
                    <span class="text-lg font-bold text-white">HotorNot Account Transfer</span>
                    <div></div>
                </div>
            </Title>
            <div class="px-8 w-full sm:w-7/12">
                <PrincipalInfoView />
            </div>
        </div>
    }
}
