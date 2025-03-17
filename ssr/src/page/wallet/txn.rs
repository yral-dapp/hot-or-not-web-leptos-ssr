use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_params;
use yral_canisters_common::utils::transaction::{TxnDirection, TxnInfoType, TxnInfoWallet};

use crate::{page::token::info::TokenKeyParam, utils::time::parse_ns_to_datetime};

fn direction_to_icon(direction: TxnDirection) -> &'static icondata_core::IconData {
    use TxnDirection::*;
    match direction {
        Transaction => icondata::LuArrowLeftRight,
        Added => icondata::FaArrowDownSolid,
        Deducted => icondata::FaArrowUpSolid,
    }
}

fn txn_info_to_icon(txn_info: TxnInfoType) -> &'static icondata_core::IconData {
    let direction = TxnDirection::from(txn_info);
    direction_to_icon(direction)
}

#[component]
pub fn TxnView(
    info: TxnInfoWallet,
    #[prop(optional)] _ref: NodeRef<html::Div>,
    symbol: String,
) -> impl IntoView {
    let params = use_params::<TokenKeyParam>();
    let direction = TxnDirection::from(info.tag);
    let bal_res = format!(
        "{}{}",
        match direction {
            TxnDirection::Added => "+",
            TxnDirection::Deducted => "-",
            TxnDirection::Transaction => "",
        },
        info.amount.humanize_float_truncate_to_dp(2)
    );

    view! {
        <div _ref=_ref class="w-full flex flex-col gap-8 bg-neutral-800 rounded-[4px] py-3 px-2">
            <div class="flex items-center gap-2">
                <div class="flex items-center gap-2">
                    {
                        match direction{
                            TxnDirection::Added => {
                                view! {
                                    <div class="flex items-center justify-center w-6 h-6 p-1 rounded-full shrink-0 bg-[#1EC9811A] text-[#158F5C]">
                                        <Icon icon=txn_info_to_icon(info.tag) />
                                    </div>
                                }
                            },
                            TxnDirection::Deducted => {
                                view! {
                                    <div class="flex items-center justify-center w-6 h-6 rounded-full shrink-0 text-red-600 bg-red-600/5 p-1">
                                        <Icon icon=txn_info_to_icon(info.tag) />
                                    </div>
                                }
                            },
                            TxnDirection::Transaction => {
                                view! {
                                    <div class="flex items-center justify-center w-6 h-6 rounded-full shrink-0 text-neutral-500 bg-neutral-700 p-1">
                                        <Icon icon=txn_info_to_icon(info.tag) />
                                    </div>
                                }
                            },
                        }
                    }
                </div>
                <div class="font-medium text-neutral-50">
                    {info.tag.to_text()}
                </div>
                <div class={format!("font-bold text-lg {}", if direction == TxnDirection::Added {
                    "text-green-600"
                } else {
                    "text-neutral-50"
                })}>
                    <span>{bal_res}</span>
                    <span class="text-[8px]">(${symbol})</span>
                </div>
            </div>
            <div class="flex flex-col gap-3 text-[#A3A3A3] text-sm">
                {
                    move || {
                        match info.tag{
                            TxnInfoType::Mint { to } => {
                                match params.get(){
                                    Ok(_) => None,
                                    Err(_) => Some(view! {
                                        <div class="flex flex-col gap-2">
                                            <div>To</div>
                                            <div class="underline line-clamp-1">{to.to_string()}</div>
                                        </div>
                                    })
                                }
                            },
                            TxnInfoType::Burn { from } => {
                                match params.get(){
                                    Ok(_) => None,
                                    Err(_) => Some(view! {
                                        <div class="flex flex-col gap-2">
                                            <div>From</div>
                                            <div class="underline line-clamp-1">{from.to_string()}</div>
                                        </div>
                                    })
                                }
                            },
                            TxnInfoType::Received { from } => Some(view! {
                                <div class="flex flex-col gap-2">
                                    <div>From</div>
                                    <div class="underline line-clamp-1">{from.to_string()}</div>
                                </div>
                            }),
                            TxnInfoType::Sent { to } => Some(view! {
                                <div class="flex flex-col gap-2">
                                    <div>To</div>
                                    <div class="underline line-clamp-1">{to.to_string()}</div>
                                </div>
                            }),
                            TxnInfoType::Transfer { from, to } => Some(view! {
                                <div class="flex flex-col gap-3">
                                    <div class="flex flex-col gap-2">
                                        <div>From</div>
                                        <div class="underline line-clamp-1">{from.to_string()}</div>
                                    </div>
                                    <div class="flex flex-col gap-2">
                                        <div>To</div>
                                        <div class="underline line-clamp-1">{to.to_string()}</div>
                                    </div>
                                </div>
                            })
                        }
                    }
                }    
                <div class="line-clamp-1 text-[#525252]">
                    {parse_ns_to_datetime(info.timestamp).ok()}
                </div>
            </div>
        </div>
    }
}

pub mod provider {

    use candid::Principal;
    use yral_canisters_common::{
        cursored_data::{transaction::IndexOrLedger, CursoredDataProvider},
        Canisters,
    };

    use super::*;

    pub(crate) fn get_history_provider(
        canisters: Canisters<false>,
        source: IndexOrLedger,
        decimals: u8,
    ) -> impl CursoredDataProvider<Data = TxnInfoWallet> + Clone {
        #[cfg(feature = "mock-wallet-history")]
        {
            _ = canisters;
            _ = source;
            _ = decimals;
            mock::MockHistoryProvider
        }
        #[cfg(not(feature = "mock-wallet-history"))]
        {
            use yral_canisters_common::cursored_data::transaction::TxnHistory;
            TxnHistory {
                canisters,
                source,
                decimals,
            }
        }
    }

    #[cfg(feature = "mock-wallet-history")]
    mod mock {
        use std::convert::Infallible;

        use rand_chacha::{
            rand_core::{RngCore, SeedableRng},
            ChaCha8Rng,
        };
        use yral_canisters_common::{
            cursored_data::PageEntry,
            utils::{time::current_epoch, token::balance::TokenBalance},
        };

        use super::*;

        #[derive(Clone, Copy)]
        pub struct MockHistoryProvider;

        fn tag_from_u32(v: u32) -> TxnInfoType {
            match v % 4 {
                0 => TxnInfoType::Mint {
                    to: Principal::anonymous(),
                },
                1 => TxnInfoType::Burn {
                    from: Principal::anonymous(),
                },
                2 => TxnInfoType::Received {
                    from: Principal::anonymous(),
                },
                3 => TxnInfoType::Sent {
                    to: Principal::anonymous(),
                },
                4 => TxnInfoType::Transfer {
                    from: Principal::anonymous(),
                    to: Principal::anonymous(),
                },
                _ => unreachable!(),
            }
        }
        impl CursoredDataProvider for MockHistoryProvider {
            type Data = TxnInfoWallet;
            type Error = Infallible;

            async fn get_by_cursor(
                &self,
                from: usize,
                end: usize,
            ) -> Result<PageEntry<TxnInfoWallet>, Infallible> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                let data = (from..end)
                    .map(|_| TxnInfoWallet {
                        amount: TokenBalance::new_cdao((rand_gen.next_u64() % 3001).into()),
                        timestamp: rand_gen.next_u64(),
                        tag: tag_from_u32(rand_gen.next_u32()),
                        id: rand_gen.next_u64(),
                    })
                    .collect();
                Ok(PageEntry { data, end: false })
            }
        }
    }
}
