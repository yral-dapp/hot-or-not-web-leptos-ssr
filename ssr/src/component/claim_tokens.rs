use candid::Principal;
use leptos::*;

use crate::{
    state::canisters::authenticated_canisters, utils::route::failure_redirect,
    utils::token::claim_tokens_from_first_neuron_if_required,
};

#[component]
pub fn ClaimTokensOrRedirectError(token_root: Principal) -> impl IntoView {
    let auth_cans = authenticated_canisters();
    let claim_res = auth_cans.derive(
        || (),
        move |cans_wire, _| async move {
            log::debug!("Claim token for {token_root}");
            claim_tokens_from_first_neuron_if_required(cans_wire?, token_root).await
        },
    );

    view! {
        <Suspense>
        {move || claim_res().map(|res| {
            match res {
                Ok(_) => (),
                Err(e) => {
                    failure_redirect(e);
                }
            }
        })}
        </Suspense>
    }
}
