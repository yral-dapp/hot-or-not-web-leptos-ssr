use std::collections::VecDeque;

use leptos::*;
use leptos_icons::*;
use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::prelude::*;
use crate::component::icons::chevron_left_icon::ChevronLeftIcon;
use crate::component::icons::pump_ai_logo::PumpAILogo;
use crate::component::icons::send_icon::SendIcon;
use crate::component::shimmer::Shimmer;

use crate::{
    try_or_redirect,
    utils::token::icpump::{
        get_pumpai_results, get_pumpai_results_contextual, ICPumpChatInteraction, TokenListItem,
    },
};

const QUERY_LIST: [&str; 3] = [
    "Dog meme token",
    "Show tokens, latest created first",
    "Animal tokens",
];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: i32) -> i32;
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ICPumpAiChatItem {
    UserItem {
        query: String,
    },
    ResponseItem {
        response: String,
        tokens: Vec<TokenListItem>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICPumpAiChat {
    pub items: VecDeque<ICPumpAiChatItem>,
    pub rag_data: String,
    pub interactions: Vec<ICPumpChatInteraction>,
}


#[component]
pub fn ICPumpAi() -> impl IntoView {
    let page_no = create_rw_signal(1);
    let query = create_rw_signal("".to_string());
    let chat = create_rw_signal(ICPumpAiChat {
        items: VecDeque::new(),
        rag_data: "".to_string(),
        interactions: vec![],
    });

    let search_action = create_action(move |()| async move {
        page_no.set(3);
        let q = query.get();

        chat.update(|c| {
            c.items
                .push_front(ICPumpAiChatItem::UserItem { query: q.clone() })
        });

        if chat.with(|c| c.interactions.is_empty()) {
            let results = get_pumpai_results(q.clone()).await;
            let results = try_or_redirect!(results);

            chat.update(|c| {
                c.rag_data = results.rag_data;
                c.items.push_front(ICPumpAiChatItem::ResponseItem {
                    response: results.text.clone(),
                    tokens: results.items,
                });

                let interaction = ICPumpChatInteraction {
                    query: q.clone(),
                    response: results.text,
                };
                c.interactions.push(interaction);
            });
        } else {
            let results = get_pumpai_results_contextual(
                q.clone(),
                chat.get().interactions.clone(),
                chat.get().rag_data.clone(),
            )
            .await;
            let results = try_or_redirect!(results);

            chat.update(|c| {
                c.items.push_front(ICPumpAiChatItem::ResponseItem {
                    response: results.text.clone(),
                    tokens: vec![],
                });

                let interaction = ICPumpChatInteraction {
                    query: q.clone(),
                    response: results.text,
                };
                c.interactions.push(interaction);
            });
        }

        query.set("".to_string());
    });

    let reset_state = create_action(move |()| async move {
        query.set("".to_string());
        chat.set(ICPumpAiChat {
            items: VecDeque::new(),
            rag_data: "".to_string(),
            interactions: vec![],
        });
    });

    view! {
        <div class="h-screen w-screen block text-white bg-[#111212]">
            <div
                class="max-w-md flex flex-col relative w-full mx-auto h-full"
                class:justify-center=move || page_no.get() != 2
                class:px-8=move || page_no.get() != 3
                class:px-4=move || page_no.get() == 3
            >

                {move || {
                    match page_no.get() {
                        1 => {
                            view! {
                                <ICPumpAiPage1
                                    query=query
                                    page_no=page_no
                                    search_action=search_action
                                />
                            }
                                .into_view()
                        }
                        2 => {
                            view! {
                                <ICPumpAiPage2
                                    query=query
                                    page_no=page_no
                                    search_action=search_action
                                    reset_state=reset_state
                                />
                            }
                                .into_view()
                        }
                        3 => {
                            view! {
                                <ICPumpAiPage3
                                    query=query
                                    chat=chat
                                    page_no=page_no
                                    search_action=search_action
                                    reset_state=reset_state
                                />
                            }
                                .into_view()
                        }
                        _ => view! { <></> }.into_view(),
                    }
                }}

            </div>
        </div>
    }
}


#[component]
pub fn ICPumpAiPage1(
    query: RwSignal<String>,
    page_no: RwSignal<i32>,
    search_action: Action<(), ()>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center gap-3">
            <PumpAILogo animate=true classes="h-18 w-18".to_string() />
            <div class="font-kumbh font-semibold text-3xl text-center">
                Welcome to <br />Pump AI
            </div>
            <SearchInput
                on_submit=move |q| {
                    query.set(q);
                    search_action.dispatch(());
                }
                on_focus=move || {
                    page_no.set(2);
                }
                query=query
            />
        </div>
        <div class="flex flex-col pt-20 gap-2">
            <div class="text-[#505156]">Try these:</div>
            <For
                each=move || QUERY_LIST
                key=|t| t.to_owned()
                children=move |token: &str| {
                    view! {
                        <button
                            on:click=move |_| {
                                query.set(token.to_string());
                                page_no.set(2);
                                search_action.dispatch(());
                            }
                            class="border-[#1B1D22] transition-colors hover:bg-zinc-800 active:bg-zinc-900 text-sm text-left font-medium py-2 px-4 border w-full"
                        >
                            {token}
                        </button>
                    }
                }
            />

        </div>
    }
}

#[component]
pub fn ICPumpAiPage2(
    query: RwSignal<String>,
    page_no: RwSignal<i32>,
    search_action: Action<(), ()>,
    reset_state: Action<(), ()>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();

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
        <Header on_back=move || {
            reset_state.dispatch(());
            page_no.set(1);
        } />
        <div class="flex flex-col pt-5 gap-2">
            <SearchInput
                classes="mt-20".to_string()
                query=query
                on_focus=move || {}
                on_submit=move |q| {
                    query.set(q);
                    search_action.dispatch(());
                }
            />
            <For
                each=move || QUERY_LIST
                key=|t| t.to_owned()
                children=move |token: &str| {
                    view! {
                        <button
                            on:click=move |_| {
                                query.set(token.to_string());
                                page_no.set(2);
                                search_action.dispatch(());
                            }
                            class="border-[#1B1D22] transition-colors hover:bg-zinc-800 active:bg-zinc-900 text-sm text-left font-medium py-2 px-4 border w-full"
                        >
                            {token}
                        </button>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn ICPumpAiPage3(
    query: RwSignal<String>,
    chat: RwSignal<ICPumpAiChat>,
    page_no: RwSignal<i32>,
    search_action: Action<(), ()>,
    reset_state: Action<(), ()>,
) -> impl IntoView {
    view! {
        <Header on_back=move || {
            reset_state.dispatch(());
            page_no.set(1);
        } />
        <div class="grow flex gap-4 flex-col-reverse mt-12 overflow-y-auto py-4">
            {move || {
                if search_action.pending().get() {
                    return view! {
                        <div class="flex w-1/2 items-center justify-start">
                            <Shimmer />
                        </div>
                    };
                }
                view! { <div class="invisible" /> }
            }}
            <For
                each=move || chat.get().items.clone()
                key=|item| item.clone()
                children=move |item: ICPumpAiChatItem| {
                    match item {
                        ICPumpAiChatItem::UserItem { query } => {
                            view! {
                                <div class="flex flex-col gap-2 relative w-full items-end pl-8">
                                    <div class="w-fit px-4 py-2 rounded-xs bg-[#202125]">
                                        {query}
                                    </div>
                                </div>
                            }
                        }
                        ICPumpAiChatItem::ResponseItem { response, tokens } => {
                            view! {
                                <div class="flex flex-col gap-2 relative w-full items-start pr-4">
                                    <div class="w-fit px-4 py-2 rounded-xs">
                                        <MarkdownRenderer text=response />
                                    </div>
                                    <ICPumpAiTokenListing tokens=tokens />
                                </div>
                            }
                        }
                    }
                }
            />

        </div>
        <div class="pb-4">
            <SearchInput
                query=query
                on_focus=move || {}
                on_submit=move |q| {
                    query.set(q);
                    search_action.dispatch(());
                }
            />
        </div>
    }
}

#[component]
pub fn SearchInput(
    #[prop(optional, default = "".to_string())] classes: String,
    on_submit: impl Fn(String) + 'static,
    on_focus: impl Fn() + 'static,
    query: RwSignal<String>, 
) -> impl IntoView {

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                on_submit(query.get());
            }
            class=format!("bg-[#202125] w-full rounded-sm relative {}", classes)
        >
            <input
                on:input=move |ev| {
                    let q = event_target_value(&ev);
                    query.set(q);
                }
                on:focus=move |_| {
                    on_focus();
                }
                value=move || query.get()
                placeholder="Ask anything"
                class="bg-transparent focus:outline-none pl-4 py-2 pr-12 w-full placeholder:text-[#505156]"
            />

            <button
                type="submit"
                disabled=query.get().is_empty()
                class="pr-2 absolute transition-opacity right-0 inset-y-0"
            >
                <SendIcon filled=!query.get().is_empty() classes="w-6 h-6".to_string() />
            </button>
        </form>
    }
}


#[component]
pub fn Header(on_back: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div class="bg-black z-[4] absolute top-0 w-full select-none inset-x-0">
            <div class="flex items-center justify-center relative gap-3 px-4 py-3 mx-auto max-w-md">
                <PumpAILogo classes="h-5 w-5".to_string() />
                <div class="text-xl font-semibold">Pump AI</div>
                <button
                    class="absolute z-[5] left-0 px-4 h-full"
                    on:click=move |_| {
                        on_back();
                    }
                >
                    <ChevronLeftIcon classes="w-4 h-4".to_string() />
                </button>
            </div>
        </div>
    }
}


#[component]
pub fn ICPumpAiToken(details: TokenListItem) -> impl IntoView {
    view! {
        <a
            href=details.link
            class="text-xs w-full p-2 flex gap-2 border border-gray-900 bg-transparent hover:bg-white/10 active:bg-white/5"
        >
            <div class="relative">
                <img
                    src=details.logo
                    class=move || {
                        let mut classes = "w-[5.5rem] shrink-0 h-[5.5rem]".to_string();
                        if details.is_nsfw {
                            classes.push_str(" blur-md");
                        }
                        classes
                    }
                />
                <Show when=move || details.is_nsfw>
                    <div class="absolute inset-0 flex items-center justify-center">
                        <Icon icon=icondata::AiEyeInvisibleOutlined class="w-8 h-8 text-gray-200" />
                    </div>
                </Show>
            </div>
            <div class="flex flex-col gap-1 text-left">
                <div class="flex w-full items-center justify-between gap-4">
                    <span class="shrink line-clamp-1">{details.token_name}</span>
                    <span class="shrink-0 font-bold">{details.token_symbol}</span>
                </div>
                <span class="line-clamp-4 text-gray-400">{details.description}</span>
            </div>
        </a>
    }
}

#[component]
pub fn ICPumpAiTokenListing(tokens: Vec<TokenListItem>) -> impl IntoView {
    let view_more = create_rw_signal(false);
    let tokens_stripped = tokens
        .iter()
        .take(3)
        .cloned()
        .collect::<Vec<TokenListItem>>();
    let tokens_len = tokens.len();

    let tokens_view_list = create_memo(move |_| {
        let tokens_stripped = tokens_stripped.clone();
        let tokens = tokens.clone();
        let tokens_final = if view_more.get() {
            tokens.clone()
        } else {
            tokens_stripped.clone()
        };

        view! {
            <div class="flex flex-col gap-2 relative w-full items-start pr-4">
                <For
                    each=move || tokens_final.clone()
                    key=|t| t.token_symbol.clone()
                    children=move |token: TokenListItem| {
                        view! { <ICPumpAiToken details=token /> }
                    }
                />
            </div>
        }
        .into_view()
    });

    let tokens_view = create_memo(move |_| {
        if tokens_len != 0 {
            view! {
                {tokens_view_list}
                <div class="w-full flex items-center justify-center">
                    <button
                        class="flex items-center gap-2 rounded-xs border border-[#202125] w-fit p-2"
                        on:click=move |_| view_more.update(|v| *v = !*v)
                    >
                        {move || {
                            if view_more.get() {
                                view! {
                                    <span>View less</span>
                                    <span>"↑"</span>
                                }
                            } else {
                                view! {
                                    <span>View more</span>
                                    <span>"↓"</span>
                                }
                            }
                        }}
                    </button>
                </div>
            }
            .into_view()
        } else {
            view! { <></> }.into_view()
        }
    });

    view! { <div class="flex flex-col gap-4">{tokens_view}</div> }
}


#[component]
pub fn MarkdownRenderer(text: String) -> impl IntoView {
    let parsed_markdown = create_memo(move |_| {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&text, options);

        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
    });

    view! {
        <div class="text-gray-200 text-sm pb-2">
            <div inner_html=parsed_markdown></div>
        </div>
    }
}
