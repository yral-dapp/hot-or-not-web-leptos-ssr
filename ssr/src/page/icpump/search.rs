use leptos::*;
use leptos_icons::*;

use crate::{
    component::spinner::{FullScreenSpinner, Spinner},
    utils::token::icpump::{get_token_search_results, TokenListItem},
};

const QUERY_LIST: [&str; 3] = ["coins with animals", "meme coins", "ape coins"];

#[component]
pub fn ICPumpSearch() -> impl IntoView {
    let query = create_rw_signal("".to_string());
    let query_results: RwSignal<Vec<TokenListItem>> = create_rw_signal(vec![]);

    let query_list = QUERY_LIST.to_vec();

    let search_action = create_action(move |()| async move {
        let q = query.get();
        // leptos::logging::log!("Search query: {}", q);

        let results = get_token_search_results(q).await.unwrap();
        query_results.set(results);
        query.set("".to_string());

        Ok::<_, ServerFnError>(())
    });

    view! {
        <div class="h-screen w-screen block bg-black text-white font-mono pb-12 overflow-y-scroll">
            <div class="flex flex-col gap-4 p-8">
                  <div class="text-gray-400">Search</div>
                <div
                class="hover:border-gray-600 border flex border-gray-900 relative focus-within:!border-gray-400"
                  >
                    <input class="w-screen bg-black text-white p-2 rounded-lg" type="text" placeholder="Search for a token"
                    on:input=move |ev| {
                        let q = event_target_value(&ev);
                        query.set(q);
                    } />
                    <button
                        class="absolute right-3 active:italic inset-y-0 items-center flex gap-1 group"
                        on:click=move |_| search_action.dispatch(())
                        >
                        <Icon class="text-xl" icon=icondata::AiSearchOutlined />
                    </button>
                </div>


                <div class="flex flex-col gap-4 p-8">
                        <div class="text-gray-400">Try these search prompts:</div>
                        <div class="flex items-center gap-2 flex-wrap">
                            <For
                                each=move || query_list.clone()
                                key=|t| t.to_string()
                                children=move |token_query: &str| {
                                view! {
                                    <button class="text-sm hover:underline hover:text-white/75 active:text-white/50 active:italic whitespace-nowrap" on:click=move |_| {
                                        query.set(token_query.to_string());
                                        search_action.dispatch(());
                                    }><span>"[ "</span>{token_query}<span>" ]"</span></button>
                                }
                                }
                            />
                        </div>
                </div>

                {
                    move || {
                        if search_action.pending().get() {
                            return view! {
                                <>
                                <div class="flex flex-col items-center justify-center">
                                    <div class="relative inline-block text-2xl">
                                        <span class="absolute animate-searching-a-1">"(→_→)"</span>
                                        <span class="absolute animate-searching-a-2">"(←_←)"</span>
                                    </div>
                                </div>
                                </>
                            };
                        }

                        let results = query_results.get();
                        if !results.is_empty() {
                            return view! {
                                <div class="text-gray-400 pb-2 self-start">Search results:</div>
                                <For
                                    each=move || results.clone()
                                    key=|t| t.token_symbol.clone()
                                    children=move |token: TokenListItem| {
                                    view! {
                                        <button
                                              class="text-xs w-full p-2 flex gap-2 border border-gray-900 bg-transparent hover:bg-white/10 active:bg-white/5"
                                           >
                                              <img src={token.logo} class="w-[5.5rem] shrink-0 h-[5.5rem]" />
                                              <div class="flex flex-col gap-1 text-left">
                                                 <div class="flex w-full items-center justify-between gap-4">
                                                        <span class="shrink line-clamp-1">{token.token_name}</span>
                                                        <span class="shrink-0 font-bold">${token.token_symbol}</span>
                                                 </div>
                                                 <span class="line-clamp-4 text-gray-400"
                                                        > {token.description} </span
                                                 >
                                              </div>
                                           </button>
                                    }
                                    }
                                />
                            };
                        }

                        view! {
                            <><div></div></>
                        }
                    }
                }
            </div>
        </div>
    }
}
