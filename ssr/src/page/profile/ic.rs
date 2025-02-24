use leptos::prelude::*;
use leptos_icons::*;
use yral_canisters_common::cursored_data::CursoredDataProvider;

use crate::{
    component::{
        bullet_loader::BulletLoader,
        infinite_scroller::{InferData, InfiniteScroller},
    },
    utils::profile::PROFILE_CHUNK_SZ,
};
use leptos::html;
#[component]
pub fn ProfileStream<Prov, EF, N>(
    provider: Prov,
    children: EF,
    empty_graphic: icondata::Icon,
    #[prop(into)] empty_text: String,
) -> impl IntoView
where
    Prov: CursoredDataProvider + Clone + Send + Sync + 'static,
    Prov::Data: Send + Sync,
    EF: Fn(InferData<Prov>, Option<NodeRef<html::Div>>) -> N + Clone + Send + Sync + 'static,
    N: IntoView + 'static,
{
    view! {
        <div class="flex flex-row gap-y-3 flex-wrap justify-center w-full">
            <InfiniteScroller
                provider
                fetch_count=PROFILE_CHUNK_SZ
                children
                empty_content=move || {
                    view! {
                        <div class="flex flex-col pt-9 gap-2 w-full justify-center items-center">
                            <Icon class="w-36 h-36" icon=empty_graphic />
                            <span class="text-lg text-white">{empty_text.clone()}</span>
                        </div>
                    }
                }

                custom_loader=move || {
                    view! {
                        <div class="w-full flex justify-center items-center pt-9">
                            <BulletLoader />
                        </div>
                    }
                }
            />

        </div>
    }
}
