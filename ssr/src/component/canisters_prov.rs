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
        let cans_wire = try_or_redirect_opt!((cans_res.0)()?);
        let cans = try_or_redirect_opt!(cans_wire.canisters());
        Some((children.get_value())(cans).into_view())
    };

    view! { <Suspense fallback=fallback>{loader}</Suspense> }
}

#[component]
fn DataLoader<N, EF, D, St, DF>(
    cans: Canisters<true>,
    fallback: ViewFn,
    with: DF,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn((Canisters<true>, D)) -> N + 'static + Clone,
    D: Serializable + Clone + 'static,
    St: 'static + Clone,
    DF: FnOnce(Canisters<true>) -> Resource<St, D> + 'static + Clone,
{
    let can_c = cans.clone();
    let with_res = (with)(can_c);

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

pub fn with_cans<D: Serializable + Clone + 'static, DFut: Future<Output = D> + 'static>(
    with: impl Fn(Canisters<true>) -> DFut + 'static + Clone,
) -> impl FnOnce(Canisters<true>) -> Resource<(), D> + Clone {
    move |cans: Canisters<true>| create_resource(|| (), move |_| (with.clone())(cans.clone()))
}

#[component]
pub fn WithAuthCans<N, EF, D, St, DF>(
    #[prop(into, optional)] fallback: ViewFn,
    with: DF,
    children: EF,
) -> impl IntoView
where
    N: IntoView + 'static,
    EF: Fn((Canisters<true>, D)) -> N + 'static + Clone,
    St: 'static + Clone,
    D: Serializable + Clone + 'static,
    DF: FnOnce(Canisters<true>) -> Resource<St, D> + 'static + Clone,
{
    view! {
        <AuthCansProvider fallback=fallback.clone() let:cans>
            <DataLoader
                cans
                fallback=fallback.clone()
                with=with.clone()
                children=children.clone()
            />
        </AuthCansProvider>
    }
}
