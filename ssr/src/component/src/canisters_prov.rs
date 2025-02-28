use futures::Future;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use  state::canisters::{authenticated_canisters, unauth_canisters};
use utils::{
    try_or_redirect_opt,
};
use utils::MockPartialEq;
use yral_canisters_common::Canisters;

#[component]
pub fn AuthCansProvider<N, EF>(
    #[prop(into, optional)] fallback: ViewFnOnce,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn(Canisters<true>) -> N + 'static + Clone + Send + Sync,
{
    let cans_res = authenticated_canisters();
    let children = StoredValue::new(children);

    view! {
        <Suspense fallback=fallback>
            {move || Suspend::new(async move {
                let cans_wire = try_or_redirect_opt!(cans_res.await);
                let maybe_cans = Canisters::from_wire(cans_wire, expect_context());
                let cans = try_or_redirect_opt!(maybe_cans);
                Some((children.read_value())(cans))
            })}
        </Suspense>
    }
}

pub fn with_cans<
    D: Send + Sync + Serialize + for<'x> Deserialize<'x>,
    DFut: Future<Output = Result<D, ServerFnError>> + 'static + Send,
>(
    with: impl Fn(Canisters<true>) -> DFut + 'static + Clone + Send + Sync,
) -> Resource<Result<D, ServerFnError>> {
    let auth_cans = authenticated_canisters();
    let base_cans = unauth_canisters();
    Resource::new(
        move || {
            // MockPartialEq is necessary
            // See: https://github.com/leptos-rs/leptos/issues/2661
            auth_cans.track();
            MockPartialEq(())
        },
        move |_| {
            let base_cans = base_cans.clone();
            let with = with.clone();
            async move {
                let cans_wire = auth_cans.await?;
                let cans = Canisters::from_wire(cans_wire, base_cans)?;
                with(cans).await
            }
        },
    )
}