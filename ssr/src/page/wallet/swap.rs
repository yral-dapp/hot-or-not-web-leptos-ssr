#[derive(Clone)]
struct SwapToken {
    name: String,
    icon: String,
    symbol: String,
}

use html::Div;
use leptos::*;
use leptos_use::on_click_outside;
use rust_decimal::prelude::Signed;

use crate::component::icons::chevron_left_icon::ChevronLeftIcon;
use crate::component::icons::arrow_down_icon::ArrowDownIcon;
use crate::component::icons::wallet_icon::WalletIcon;
use crate::component::icons::chevron_right_icon::ChevronRightIcon;
use crate::component::icons::arrow_right_long_icon::ArrowRightLongIcon;
use crate::component::icons::close_icon::CloseIcon;
use crate::component::buttons::Button;

#[component]
pub fn SwapPage() -> impl IntoView {

  let (wallet_balance, set_wallet_balance) = create_signal(100.0);

  let swap_from = SwapToken {
    name: "Hot".to_string(),
    icon: "https://picsum.photos/200".to_string(),
    symbol: "HOT".to_string(),
  };

  let profile_image = "https://picsum.photos/200";
  let profile_id = "mqxpy-vp4st-vhw6p-poxzk-i363n-y4fag-v37dn-22pu7";

  
  let (from_amount, set_from_amount) = create_signal(0);
  let (to_amount, set_to_amount) = create_signal(0);
  let (selected_token, set_selected_token) = create_signal(None);
  let (selected_token_wallet_balance, set_selected_token_wallet_balance) = create_signal(0);
  let (selected_token_multiplier, set_selected_token_multiplier) = create_signal(0.08);
  
  let (is_amount_invalid, set_is_amount_invalid) = create_signal(false);

  let request_swap = move || {
    // Create swap request
  };

  let (show_select_coin_popup, set_show_select_coin_popup) = create_signal(false);
  let (show_confirm_popup, set_show_confirm_popup) = create_signal(false);
  
    view! {
        {move || {
            if show_confirm_popup.get() {
                view! { <><ConfirmPopup /></> }
            } else {
                view! { <><div class="invisible" /></> }
            }
        }}

        {move || {
          if show_select_coin_popup.get() {
                view! { <><SelectCoinPopup /></> }
            } else {
                view! { <><div class="invisible" /></> }
            }
        }}

        <div class="h-screen w-screen block text-white bg-black font-kumbh">
            <Header />
            <div class="h-full w-full pt-20 pb-8">
                <div class="flex flex-col items-center justify-center gap-3 max-w-md mx-auto px-4">
                    <div class="flex flex-col gap-2">
                        <div class="text-[#A0A1A6]">"From Principal ID:"</div>
                        <div class="border border-[#3A3A3A] rounded-lg flex py-2 px-3 items-center gap-2">
                            <img src=profile_image alt="avatar" class="w-8 h-8 rounded-full" />
                            <div class="text-lg font-medium">{profile_id}</div>
                        </div>
                    </div>
                    <div class="w-full flex flex-col gap-2">
                        <label class="flex bg-[#191919] rounded-lg flex-col px-4 gap-2 py-4 relative hover:bg-zinc-900 focus-within:bg-zinc-900 active:bg-zinc-800">
                            <span class="text-sm font-medium text-[#A0A1A6]">"SELL"</span>
                            <input
                                type="number"
                                on:input=move |e| {
                                    set_from_amount(event_target_value(&e).parse::<i32>().unwrap_or(0));
                                }
                                placeholder="0"
                                class="focus:outline-none font-bold text-3xl mr-28 py-2 bg-transparent placeholder:text-zinc-500 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                            />
                            <div class="text-sm flex items-center justify-end gap-2 font-mono text-zinc-300">
                                <WalletIcon classes="w-4 h-4".to_string() />
                                <div>{wallet_balance.get()}</div>
                            </div>
                            <div class="absolute inset-x-0 -bottom-8 z-[4] flex items-center justify-center p-2">
                                <div class="bg-[#191919] border-black border-[6px] rounded-[10px] p-2 text-[#A0A1A6]">
                                    <ArrowDownIcon classes="w-4 h-4".to_string() />
                                </div>
                            </div>
                            <div class="absolute inset-y-0 right-2 flex flex-col items-center justify-center">
                                <div class="gap-2 bg-[#202125] rounded-[4px] px-3 py-2 flex items-center justify-center font-bold select-none">
                                    <img
                                        src=swap_from.icon
                                        alt=swap_from.name.clone()
                                        class="w-6 h-6 rounded-full object-cover"
                                    />
                                    <span>{swap_from.name}</span>
                                </div>
                            </div>
                        </label>

                        <label class="flex bg-[#191919] rounded-lg flex-col px-4 gap-2 py-4 relative hover:bg-zinc-900 focus-within:bg-zinc-900 active:bg-zinc-800">
                            <span class="text-sm font-medium text-[#A0A1A6]">BUY</span>
                            <input
                                type="number"
                                on:input=move |e| {
                                    set_to_amount(event_target_value(&e).parse::<i32>().unwrap_or(0));
                                }
                                value=move || to_amount.get()
                                placeholder="0"
                                class=format!(
                                    "focus:outline-none font-bold text-3xl py-2 mr-28 bg-transparent placeholder:text-zinc-500 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none {}",
                                    if is_amount_invalid.get() { "text-[#F14331]" } else { "" },
                                )
                            />
                            <div class=format!(
                                "text-sm flex items-center justify-end gap-2 font-mono text-zinc-300 {}",
                                if selected_token.get().is_none() { "opacity-0" } else { "" },
                            )>
                                <WalletIcon classes="w-4 h-4".to_string() />
                                <div>{selected_token_wallet_balance.get()}</div>
                            </div>

                            <div class="absolute inset-y-0 right-2 z-[5] flex flex-col items-center justify-center">
                                <button
                                    on:click=move |_| {
                                      set_show_select_coin_popup(true)
                                    }
                                    class=format!(
                                        "border border-zinc-800 px-3 py-2 flex items-center gap-2 text-sm font-bold {}",
                                        if selected_token.get().is_none() {
                                            "bg-white hover:bg-white/80 active:bg-white/60 text-black rounded-full"
                                        } else {
                                            "bg-[#202125] rounded-[4px]"
                                        },
                                    )
                                >
                                    {if (selected_token.get().is_some()) {
                                        view! {
                                          <>
                                            <img
                                                src=selected_token.get().unwrap().icon
                                                alt=selected_token.get().unwrap().name.clone()
                                                class="w-6 h-6 rounded-full object-cover"
                                            />
                                            <span>{selected_token.get().unwrap().name.clone()}</span>
                                            <ChevronRightIcon classes="w-4 h-4 rotate-90".to_string() />
                                            </>
                                        }
                                    } else {
                                        view! { 
                                          <>
                                            <span>Select token</span>
                                          </>
                                        }
                                    }}
                                </button>
                            </div>
                        </label>

                        <div class="flex flex-col gap-1 text-sm font-medium text-[#A0A1A6] pb-5 pt-1">
                            {if (is_amount_invalid.get()) {
                                view! { <div class="text-[#F14331]">"Amount is too high"</div> }
                            } else {
                                view! {
                                    <div>
                                        {format!("1 {} = {} {}", swap_from.name.clone(), selected_token_multiplier.get(), selected_token.get().unwrap().name.clone())}
                                    </div>
                                }
                            }}
                            <div>
                                {format!("1 {} = {} {}", swap_from.name.clone(), selected_token_multiplier.get(), selected_token.get().unwrap().name.clone())}
                            </div>
                        </div>

                        <Button
                            on_click=move || {
                              request_swap()
                            }
                            disabled=(!selected_token.get().is_some() || !from_amount.get().is_positive())
                        >
                            "Request swap"
                        </Button>
                    </div>
                </div>
            </div>
        </div>

        
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="bg-black z-[4] fixed top-0 w-full select-none inset-x-0">
            <div class="flex items-center justify-center relative gap-3 p-4 mx-auto max-w-md">
                <div class="text-xl font-semibold">"Swap"</div>
                <a href="/wallet" class="absolute z-[5] left-0 px-4">
                    <ChevronLeftIcon classes="w-4 h-4".to_string() />
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn SelectCoinPopup() -> impl IntoView {
  let target = NodeRef::<Div>::new();
  on_click_outside(target, move |event| {
    // Close popup
  });

  let (coin_search_query, set_coin_search_query) = create_signal("".to_string());
  let (list_of_coins, set_list_of_coins) = create_signal(vec![
    SwapToken {
      name: "Hot".to_string(),
      icon: "https://picsum.photos/200".to_string(),
      symbol: "HOT".to_string(),
    }
  ]);

    view! {
        <div class="fixed z-[999] inset-0">
            <div
                node_ref=target
                class="max-w-md select-none mx-auto h-full w-full z-[999] py-20 px-4"
            >
                <div class="bg-[#131313] flex flex-col py-4 text-white h-full w-full overflow-y-auto shadow-md rounded-lg">
                    <div class="px-4 pb-4 border-b border-zinc-900">
                        <input
                            value=move || coin_search_query.get()
                            on:input=move |e| {
                                set_coin_search_query(event_target_value(&e).to_string());
                            }
                            type="text"
                            placeholder="Search for a token"
                            class="w-full px-4 py-3 text-sm focus:outline-none bg-[#202125] rounded-[4px]"
                        />
                    </div>
                    {if (list_of_coins.get().len() > 0) {
                        list_of_coins
                            .get()
                            .iter()
                            .map(|coin| {
                                view! {
                                  <>
                                    <button
                                        class="p-4 flex items-center gap-2 border-zinc-900 hover:bg-zinc-900 active:bg-zinc-800 border-t"
                                        on:click=move |_| {
                                            //selected_token.set(Some(coin.clone()));
                                            //show_select_coin_popup.set(false);
                                        }
                                    >
                                        <div class="w-5 h-5 bg-zinc-700 rounded-full overflow-hidden">
                                            <img
                                                src=coin.icon
                                                alt=coin.name
                                                class="h-full w-full rounded-full overflow-hidden object-cover"
                                            />
                                        </div>

                                        <div class="font-bold">{coin.name.clone()}</div>
                                        <div class="text-zinc-400">{coin.symbol.clone()}</div>
                                    </button>
                                    </>
                                }
                            })
                    } else {
                        view! { 
                          <>
                            <div class="text-zinc-500 text-center py-4">No results found</div> 
                          </>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
    
#[component]
fn ConfirmPopup() -> impl IntoView {
  let target = NodeRef::<Div>::new();
  on_click_outside(target, move |event| {
    // Close popup
  });

  let (from_token, set_from_token) = create_signal(SwapToken {
    name: "Hot".to_string(),
    icon: "https://picsum.photos/200".to_string(),
    symbol: "HOT".to_string(),
  });
  let (to_token, set_to_token) = create_signal(SwapToken {
    name: "Rock".to_string(),
    icon: "https://picsum.photos/200".to_string(),
    symbol: "ROCK".to_string(),
  });

  view! {
      <div class="fixed z-[999] inset-0">
          <div
              node_ref=target
              class="max-w-md select-none flex flex-col items-center justify-center h-full bg-black/50 backdrop-blur-sm mx-auto w-full z-[999] py-20 px-4"
          >
              <div class="bg-[#191919] relative flex flex-col gap-1 text-white items-center shadow-md rounded-sm p-8">
                  <button
                      on:click=move |_| {
                        // show_confirm_popup.set(false)
                      }
                      class="absolute top-3 right-3 z-[999] p-2 bg-[#3A3A3A] rounded-full"
                  >
                      <CloseIcon classes="w-3 h-3".to_string() />
                  </button>
                  <div class="flex items-center gap-4 pt-8">
                      <img
                          src=from_token.get().icon.clone()
                          alt=from_token.get().name.clone()
                          class="w-20 h-20 rounded-full"
                      />
                      <ArrowRightLongIcon classes="w-8 text-zinc-500".to_string() />
                      <img
                          src=to_token.get().icon.clone()
                          alt=to_token.get().name.clone()
                          class="w-20 h-20 rounded-full"
                      />
                  </div>
                  <div class="text-xl font-bold text-center pt-3">
                      {format!("{} to {}", from_token.get().name, to_token.get().name)}
                  </div>
                  <div class="text-xl font-bold text-center text-[#FFC33A]">
                      "Swap request sent successfully!"
                  </div>
              </div>
          </div>
      </div>
  }
}