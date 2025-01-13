use super::bullet_loader::BulletLoader;
use leptos::{html::ElementType, prelude::*};
use leptos_use::{
    core::{IntoElementsMaybeSignal, SignalVecMarker},
    use_intersection_observer_with_options, UseIntersectionObserverOptions,
};
use std::marker::PhantomData;
use yral_canisters_common::cursored_data::{CursoredDataProvider, KeyedData, PageEntry};

pub(crate) type InferData<T> = <T as CursoredDataProvider>::Data;

/// Infinite scroller which fetches data from provider
/// and renders children
/// It will fetch new data whenever the end of the list is reached
/// also shows a loader while fetching data
/// node_ref MUST be passed to the root element if passed to the `children` fn
#[component]
pub(crate) fn InfiniteScroller<Prov, EF, N, RootNode>(
    provider: Prov,
    fetch_count: usize,
    children: EF,
    #[prop(optional, into)] empty_content: ViewFn,
    #[prop(optional, into)] custom_loader: Option<ViewFn>,
    #[prop(optional)] _iv: PhantomData<N>,
    #[prop(optional)] _rn: PhantomData<RootNode>,
) -> impl IntoView
where
    RootNode: ElementType + Clone + 'static,
    NodeRef<RootNode>: IntoElementsMaybeSignal<leptos::web_sys::Element, SignalVecMarker>,
    Prov: CursoredDataProvider + Clone + 'static + Send + Sync,
    <Prov as CursoredDataProvider>::Data: Send + Sync,
    EF: Fn(InferData<Prov>, Option<NodeRef<RootNode>>) -> N + Clone + 'static + Send + Sync,
    N: IntoView + 'static,
{
    let data = RwSignal::new(Vec::<InferData<Prov>>::new());
    let end = RwSignal::new(false);
    let cursor = RwSignal::new(0);

    let fetch_res = Resource::new(cursor, move |cursor| {
        let provider = provider.clone();
        async move {
            let PageEntry {
                data: mut fetched,
                end: list_end,
            } = match provider.get_by_cursor(cursor, cursor + fetch_count).await {
                Ok(t) => t,
                Err(e) => {
                    log::warn!("failed to fetch data err {e}");
                    PageEntry {
                        data: vec![],
                        end: true,
                    }
                }
            };
            data.try_update(|t| t.append(&mut fetched));
            end.try_set(list_end);
        }
    });
    let upper_data = move || {
        data.with(|data| {
            data.iter()
                .take(data.len().saturating_sub(1))
                .cloned()
                .collect::<Vec<_>>()
        })
    };
    let last_data = move || data.with(|data| data.last().cloned());
    let last_elem = NodeRef::<RootNode>::new();

    use_intersection_observer_with_options(
        last_elem,
        move |entry, _| {
            let Some(_visible) = entry.first().filter(|entry| entry.is_intersecting()) else {
                return;
            };
            if end.get_untracked() {
                return;
            }
            cursor.update(|c| *c += fetch_count);
        },
        UseIntersectionObserverOptions::default().thresholds(vec![0.1]),
    );
    let data_loading = move || fetch_res.with(|d| d.is_none());
    let children = StoredValue::new(children);
    let loader = custom_loader.unwrap_or_else(|| BulletLoader.into());

    view! {
        <For
            each=upper_data
            key=KeyedData::key
            children=move |data| (children.get_value())(data, None)
        />
        {move || { last_data().map(|info| (children.get_value())(info, Some(last_elem))) }}

        <Show when=data_loading>{loader.run()}</Show>
        <Show when=move || {
            !data_loading() && data.with(|d| d.is_empty())
        }>{empty_content.run()}</Show>
    }.into_any()
}
