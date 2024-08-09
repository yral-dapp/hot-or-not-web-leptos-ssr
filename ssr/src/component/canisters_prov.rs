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
    let loader = move || {
        let cans = cans_res()?.ok()?;
        Some((children.get_value())(cans).into_view())
    };
    let fallback = store_value(fallback);

    view! {
        <Suspense fallback=fallback
            .get_value()>{move || loader().unwrap_or_else(|| fallback.get_value().run())}</Suspense>
    }
}

#[component]
fn DataLoader<N, EF, D, St, DF>(
    cans: Canisters<true>,
    fallback: StoredValue<ViewFn>,
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
        <Suspense fallback=fallback
            .get_value()>
            {move || {
                with_res()
                    .map(move |d| (children.get_value())((cans.get_value(), d)).into_view())
                    .unwrap_or_else(move || fallback.get_value().run())
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
    let cans_res = authenticated_canisters();
    let fallback = store_value(fallback);
    let children = store_value(children);
    let with = store_value(with);

    let loader = move || {
        let cans = cans_res()?.ok()?;
        Some(
            view! { <DataLoader cans fallback with=with.get_value() children=children.get_value()/> },
        )
    };

    view! {
        <Suspense fallback=fallback
            .get_value()>
            {move || loader().unwrap_or_else(move || fallback.get_value().run())}
        </Suspense>
    }
}
