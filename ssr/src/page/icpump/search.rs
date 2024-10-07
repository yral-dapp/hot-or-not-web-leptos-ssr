use leptos::*;
use leptos_icons::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::prelude::*;

use crate::{
    page::icpump::TokenListing,
    try_or_redirect,
    utils::token::icpump::{get_token_search_results, TokenListItem},
};

const QUERY_LIST: [&str; 3] = ["dog", "Show tokens, latest created first", "Animal token"];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: i32) -> i32;
}

#[component]
pub fn ICPumpSearchSuggestions(
    query: RwSignal<String>,
    search_action: Action<(), ()>,
) -> impl IntoView {
    let query_list = QUERY_LIST.to_vec();

    view! {
        <div class="flex flex-col gap-4 p-8">
            <div class="text-gray-400">Try these search prompts:</div>
            <ul class="block">
                {
                    query_list.iter().cloned()
                    .map(|q| {
                        let q_clone = q;
                        view! {
                            <li>
                                <p class="text-sm inline cursor-pointer pr-2 hover:underline hover:text-white/75 active:text-white/50 active:italic" on:click=move |_| {
                                    query.set(q_clone.to_string());
                                    search_action.dispatch(());
                                }>
                                    <span>"[ "</span>{q}<span>" ]"</span>
                                </p>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}

// TODO: use this when search text is to be shown
// #[component]
// pub fn MarkdownRenderer(text: String) -> impl IntoView {
//     let parsed_markdown = create_memo(move |_| {
//         let mut options = Options::empty();
//         options.insert(Options::ENABLE_STRIKETHROUGH);
//         let parser = Parser::new_ext(&text, options);
//         let mut html_output = String::new();
//         pulldown_cmark::html::push_html(&mut html_output, parser);
//         html_output
//     });

//     view! {
//         <div class="text-gray-200 pb-2 self-start">
//             <div inner_html=parsed_markdown></div>
//         </div>
//     }
// }

#[component]
pub fn ICPumpSearch() -> impl IntoView {
    let query = create_rw_signal("".to_string());
    let query_results: RwSignal<Vec<TokenListItem>> = create_rw_signal(vec![]);
    // let query_result_text = create_rw_signal("".to_string());
    let input_ref = create_node_ref::<html::Input>();

    let search_action = create_action(move |()| async move {
        let q = query.get();

        let results = get_token_search_results(q).await;
        let results = try_or_redirect!(results);

        query_results.set(results.items);
        // query_result_text.set(results.text);
    });

    create_effect(move |_| {
        if let Some(input) = input_ref.get() {
            // Focus the input
            let _ = input.focus();

            // Use setTimeout to trigger focus again after a short delay
            let closure = Closure::wrap(Box::new(move || {
                let _ = input.focus();
                input.click();
            }) as Box<dyn FnMut()>);

            setTimeout(&closure, 100);
            closure.forget(); // Prevent the closure from being dropped
        }
    });

    view! {
        <div class="h-screen w-screen block bg-black text-white font-mono pb-12 overflow-y-scroll">
            <div class="flex flex-col gap-4 p-8">
                <div class="text-gray-400">Search</div>
                <div class="relative flex items-center w-full">
                    <input
                        class="w-full bg-black text-white p-2 pr-10 rounded-lg border border-gray-900 hover:border-gray-600 focus:border-gray-400"
                        type="text"
                        placeholder="Search for a token"
                        _ref=input_ref
                        prop:value=move || query.get()
                        on:input=move |ev| {
                            let q = event_target_value(&ev);
                            query.set(q);
                        }
                        on:keypress=move |ev: ev::KeyboardEvent| {
                            if ev.key() == "Enter" {
                            search_action.dispatch(());
                            }
                        }
                        autofocus
                    />
                    <button
                        class="absolute right-2 inset-y-0 flex items-center active:italic group"
                        on:click=move |_| search_action.dispatch(())
                    >
                        <Icon class="text-xl text-gray-400 group-hover:text-gray-200" icon=icondata::AiSearchOutlined />
                    </button>
                </div>

                <ICPumpSearchSuggestions
                    query=query
                    search_action=search_action
                />

                {
                    move || {
                        if search_action.pending().get() {
                            return view! {
                                <>
                                <div class="flex flex-col items-center justify-center">
                                    <div class="relative text-2xl">
                                        <span class="absolute animate-searching-a-1">"(→_→)"</span>
                                        <span class="animate-searching-a-2">"(←_←)"</span>
                                    </div>
                                    <div class="text-gray-400">Searching...</div>
                                </div>
                                </>
                            };
                        }

                        let results = query_results.get();
                        if !results.is_empty() {
                            return view! {
                                <div class="text-gray-400 pb-2 self-start">Search results:</div>
                                // <MarkdownRenderer text=query_result_text.get() />
                                <For
                                    each=move || results.clone()
                                    key=|t| t.token_symbol.clone()
                                    children=move |token: TokenListItem| {
                                        view! {
                                            <TokenListing details=token />
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
