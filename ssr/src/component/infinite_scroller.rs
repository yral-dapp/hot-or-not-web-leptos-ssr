use super::bullet_loader::BulletLoader;
use leptos::{html::ElementDescriptor, *};
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};
use std::{error::Error, hash::Hash, marker::PhantomData};

pub struct PageEntry<T> {
    pub data: Vec<T>,
    pub end: bool,
}

/// Globally Unique key for the given type
pub trait KeyedData {
    type Key: Eq + Hash + 'static;

    fn key(&self) -> Self::Key;
}

pub(crate) trait KeyedCursoredDataProvider<T>:
    crate::component::infinite_scroller::CursoredDataProvider
{
    async fn get_by_cursor_by_key(
        &self,
        start: usize,
        end: usize,
        _user: T,
    ) -> Result<PageEntry<Self::Data>, Self::Error> {
        <Self as CursoredDataProvider>::get_by_cursor(self, start, end).await
    }
}
pub(crate) trait CursoredDataProvider {
    type Data: KeyedData + Clone + 'static;
    type Error: Error;

    async fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<Self::Data>, Self::Error>;
}

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
    RootNode: ElementDescriptor + Clone + 'static,
    Prov: CursoredDataProvider + Clone + 'static,
    EF: Fn(InferData<Prov>, Option<NodeRef<RootNode>>) -> N + Clone + 'static,
    N: IntoView + 'static,
{
    let data = create_rw_signal(Vec::<InferData<Prov>>::new());
    let end = create_rw_signal(false);
    let cursor = create_rw_signal(0);

    let fetch_res = create_resource(cursor, move |cursor| {
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
        with!(|data| data
            .iter()
            .take(data.len().saturating_sub(1))
            .cloned()
            .collect::<Vec<_>>())
    };
    let last_data = move || with!(|data| data.last().cloned());
    let last_elem = create_node_ref::<RootNode>();

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
    let data_loading = fetch_res.loading();
    let children = store_value(children);
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
    }
}
