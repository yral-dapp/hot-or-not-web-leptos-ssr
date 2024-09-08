use futures::Future;
pub use leptos::*;

use crate::{
    state::canisters::{authenticated_canisters, Canisters},
    try_or_redirect_opt,
};

#[component]
pub fn AuthCansProvider<N, EF>(
    #[prop(into, optional)] fallback: ViewFn,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn(Canisters<true>) -> N + 'static + Clone,
{
    let cans_res = authenticated_canisters();
    let children = store_value(children);
    let loader = move || {
        let cans_wire = try_or_redirect_opt!(cans_res()?);
        let cans = try_or_redirect_opt!(cans_wire?.canisters());
        Some((children.get_value())(cans).into_view())
    };

    view! { <Suspense fallback=fallback>{loader}</Suspense> }
}

#[component]
fn DataLoader<N, EF, D, DFut, DF>(
    cans: Canisters<true>,
    fallback: ViewFn,
    with: DF,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn((Canisters<true>, D)) -> N + 'static + Clone,
    DFut: Future<Output = D>,
    D: Serializable + Clone + 'static,
    DF: Fn(Canisters<true>) -> DFut + 'static + Clone,
{
    let can_c = cans.clone();
    let with_res = create_resource(
        || (),
        move |_| {
            let cans = can_c.clone();
            let with = with.clone();
            async move { (with)(cans).await }
        },
    );

    let cans = store_value(cans.clone());
    let children = store_value(children);

    view! {
        <Suspense fallback=fallback>
            {move || {
                with_res().map(move |d| (children.get_value())((cans.get_value(), d)).into_view())
            }}

        </Suspense>
    }
}

#[component]
pub fn WithAuthCans<N, EF, D, DFut, DF>(
    #[prop(into, optional)] fallback: ViewFn,
    with: DF,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn((Canisters<true>, D)) -> N + 'static + Clone,
    DFut: Future<Output = D>,
    D: Serializable + Clone + 'static,
    DF: Fn(Canisters<true>) -> DFut + 'static + Clone,
{
    view! {
        <AuthCansProvider fallback=fallback.clone() let:cans>
            <DataLoader cans fallback=fallback.clone() with=with.clone() children=children.clone()/>
        </AuthCansProvider>
    }
}
