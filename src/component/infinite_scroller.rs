use super::bullet_loader::BulletLoader;
use leptos::{html::Div, *};
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};
use std::{error::Error, hash::Hash};

pub struct PageEntry<T> {
    pub data: Vec<T>,
    pub end: bool,
}

/// Globally Unique key for the given type
pub trait KeyedData {
    type Key: Eq + Hash + 'static;

    fn key(&self) -> Self::Key;
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

type InferData<T> = <T as CursoredDataProvider>::Data;

/// Infinite scroller which fetches data from provider
/// and renders children
/// It will fetch new data whenever the end of the list is reached
/// also shows a loader while fetching data
#[component]
pub(crate) fn InfiniteScroller<Prov, EF, N>(
    provider: Prov,
    fetch_count: usize,
    children: EF,
) -> impl IntoView
where
    Prov: CursoredDataProvider + Clone + 'static,
    EF: Fn(InferData<Prov>) -> N + Clone + 'static,
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
            data.update(|t| t.append(&mut fetched));
            end.set(list_end);
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
    let last_elem = create_node_ref::<Div>();

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

    view! {
        <For each=upper_data key=KeyedData::key children=children.clone()/>
        {move || {
            last_data()
                .map(|info| {
                    view! {
                        <div _ref=last_elem class="w-full">
                            {children(info)}
                        </div>
                    }
                })
        }}

        <Show when=fetch_res.loading()>
            <BulletLoader/>
        </Show>
    }
}
