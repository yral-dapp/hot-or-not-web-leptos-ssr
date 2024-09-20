use candid::Principal;
use leptos::*;

use crate::{
    state::canisters::authenticated_canisters,
    utils::route::failure_redirect,
    utils::{time::sleep, token::claim_tokens_from_first_neuron_if_required},
};
use web_time::Duration;

#[component]
pub fn ClaimTokensOrRedirectError(token_root: Principal) -> impl IntoView {
    let auth_cans = authenticated_canisters();
    let claim_res = auth_cans.derive(
        || (),
        move |cans_wire, _| async move {
            log::debug!("Claim token for {token_root}");
            let cans_wire = cans_wire?;
            loop {
                let res =
                    claim_tokens_from_first_neuron_if_required(cans_wire.clone(), token_root).await;
                match res {
                    Ok(_) => return Ok(()),
                    Err(ServerFnError::ServerError(e)) if e.contains("PreInitializationSwap") => {
                        log::warn!("Governance is not ready. Retrying...");
                        sleep(Duration::from_secs(8)).await;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
        },
    );

    view! {
        <Suspense>
            {move || {
                claim_res()
                    .map(|res| {
                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                failure_redirect(e);
                            }
                        }
                    })
            }}

        </Suspense>
    }
}
