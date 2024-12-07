use leptos::prelude::*;

use crate::{
    component::canisters_prov::AuthCansProvider, state::auth::account_connected_reader,
    utils::notifications::get_token_for_principal,
};
use yral_canisters_common::utils::profile::ProfileDetails;

#[component]
fn NotifInnerComponent(details: ProfileDetails) -> impl IntoView {
    let (_, _) = account_connected_reader();

    // TODO: switch to Action::new_local
    let on_token_click: Action<_, _, LocalStorage> = Action::new_unsync(move |()| async move {
        get_token_for_principal(details.principal.to_string()).await;
    });

    view! {
        <h1>"YRAL Notifs for"</h1>
        <h2>{details.username_or_principal()}</h2>
        <br />
        <div class="flex flex-row gap-2 text-black">
            <button
                class="p-2 bg-gray-200 rounded-md"
                on:click=move |_| { on_token_click.dispatch(()); }
            >
                "Get Token"
            </button>
        </div>
    }
}

#[component]
pub fn Notif() -> impl IntoView {
    view! {
        <div class="h-screen w-screen grid grid-cols-1 justify-items-center place-content-center">
            <AuthCansProvider let:cans>
                <NotifInnerComponent details=cans.profile_details() />
            </AuthCansProvider>
        </div>
    }
}
