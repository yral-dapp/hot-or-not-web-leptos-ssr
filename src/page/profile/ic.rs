use std::{hash::Hash, marker::PhantomData};

use futures::Stream;
use leptos::*;

use crate::{component::bullet_loader::BulletLoader, utils::profile::PROFILE_CHUNK_SZ};

#[component]
pub fn ProfileStream<T, I: 'static, S, K, KF, N, EF>(
    base_stream: S,
    key: KF,
    children: EF,
    #[prop(optional)] _ty: PhantomData<T>,
    #[prop(optional)] _ky: PhantomData<K>,
    #[prop(optional)] _child: PhantomData<N>,
) -> impl IntoView
where
    S: Stream<Item = Vec<I>> + 'static + Unpin,
    K: Eq + Hash + 'static,
    KF: Fn(&T) -> K + 'static,
    N: IntoView + 'static,
    EF: Fn(T) -> N + 'static,
    T: (for<'a> From<&'a I>) + 'static + Clone,
{
    let chunk_loaded = create_signal_from_stream(base_stream);
    let data = create_rw_signal(Vec::<T>::new());
    let data_loaded = create_rw_signal(false);

    create_effect(move |_| {
        with!(|chunk_loaded| {
            let Some(chunk) = chunk_loaded else {
                return;
            };
            if chunk.len() < PROFILE_CHUNK_SZ {
                data_loaded.set(true);
            }
            data.update(|data| data.extend(chunk.iter().map(T::from)));
        })
    });

    view! {
        <div class="flex flex-row-reverse gap-y-3 flex-wrap-reverse justify-center w-full">
            <For each=data key children/>
        </div>
        <Show when=move || !data_loaded()>
            <BulletLoader/>
        </Show>
    }
}
