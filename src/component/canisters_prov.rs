use futures::Future;
pub use leptos::*;

use crate::state::canisters::{authenticated_canisters, Canisters};

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
    view! {
        <Suspense fallback>
            {move || {
                let cans_wire = cans_res().flatten()?;
                Some((children.get_value())(cans_wire.try_into().unwrap()))
            }}

        </Suspense>
    }
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
        <Suspense fallback>
            {move || with_res().map(move |d| (children.get_value())((cans.get_value(), d)))}
        </Suspense>
    }
}

#[component]
pub fn WithAuthCans<N, EF, D, DFut, DF>(
    #[prop(into)] fallback: ViewFn,
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
    let cans_res = authenticated_canisters();
    let fallback = store_value(fallback);
    let children = store_value(children);
    let with = store_value(with);
    view! {
        <Suspense fallback=fallback
            .get_value()>
            {move || {
                let cans_wire = cans_res().flatten()?;
                let cans: Canisters<true> = cans_wire.try_into().unwrap();
                Some(
                    view! {
                        <DataLoader
                            cans
                            fallback=fallback.get_value()
                            with=with.get_value()
                            children=children.get_value()
                        />
                    },
                )
            }}

        </Suspense>
    }
}
