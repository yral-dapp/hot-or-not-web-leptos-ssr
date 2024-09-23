use leptos::*;
use leptos_icons::*;

use crate::{
    component::{overlay::ActionTrackerPopup, token_confetti_symbol::TokenConfettiSymbol},
    page::token::create::CreateTokenCtx,
    utils::token::TokenBalance,
};

#[component]
fn SuccessPopup<ImgIV: IntoView, Img: Fn() -> ImgIV, TxtIV: IntoView, Txt: Fn() -> TxtIV>(
    img: Img,
    text: Txt,
    #[prop(into)] previous_link: MaybeSignal<String>,
    #[prop(into)] previous_text: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center w-full h-full gap-6">
            {img()} <span class="text-2xl md:text-3xl font-bold text-center">{text()}</span>
            <a
                href=previous_link
                class="w-3/4 py-4 text-lg text-center text-white bg-primary-600 rounded-full"
            >
                {previous_text}
            </a>
        </div>
    }
}

#[component]
fn CreateTokenSuccessPopup(
    #[prop(into)] token_name: String,
    #[prop(into)] img_url: String,
) -> impl IntoView {
    CreateTokenCtx::reset();
    let profile_url = "/your-profile?tab=tokens";
    view! {
        <SuccessPopup
            img=move || {
                view! {
                    <img
                        class="relative w-20 h-20 rounded-full border-2 border-primary-600 object-conver"
                        style="height:15rem; width:15rem"
                        src=img_url.clone()
                    />
                }
            }

            text=move || {
                view! {
                    Token
                    <span class="text-primary-600">{format!(" {token_name} ")}</span>
                    successfully created!
                }
            }

            previous_link=profile_url
            previous_text="Back to profile"
        />
    }
}

#[component]
fn ErrorPopup<HeadIV: IntoView, Head: Fn() -> HeadIV>(
    error: String,
    header: Head,
    #[prop(into)] previous_link: MaybeSignal<String>,
    #[prop(into)] previous_text: String,
    close_popup: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center w-full h-full gap-6">
            <div class="flex flex-row items-center justify-center bg-amber-100 text-orange-400 rounded-full p-3 text-2xl md:text-3xl">
                <Icon icon=icondata::BsExclamationTriangle/>
            </div>
            <span class="text-2xl md:text-3xl font-bold text-center">{header()}</span>
            <textarea
                prop:value=error
                disabled
                rows=3
                class="bg-black/10 text-xs md:text-sm text-red-500 w-full md:w-2/3 resize-none p-2"
            ></textarea>
            <button
                on:click=move |_| close_popup.set(true)
                class="py-3 text-lg md:text-xl w-full rounded-full bg-primary-600 text-white text-center"
            >
                Retry
            </button>
            <a
                href=previous_link
                class="py-3 text-lg md:text-xl w-full rounded-full text-black text-center bg-white border border-black"
            >
                {previous_text}
            </a>
        </div>
    }
}

#[component]
fn CreateTokenErrorPopup(
    error: String,
    token_name: MaybeSignal<String>,
    close_popup: WriteSignal<bool>,
) -> impl IntoView {
    let profile_url = String::from("/your-profile?tab=tokens");

    view! {
        <ErrorPopup
            error
            header=move || {
                let token_name = token_name.clone();
                view! {
                    Token
                    <span class="text-primary-600">
                        {move || format!(" {} ", token_name.with(|t| t.clone()))}
                    </span>
                    creation failed!
                }
            }

            previous_link=profile_url
            previous_text="Back to profile"
            close_popup
        />
    }
}

#[component]
pub fn TokenCreationPopup(
    creation_action: Action<(), Result<(), String>>,
    #[prop(into)] token_name: MaybeSignal<String>,
    #[prop(into)] img_url: MaybeSignal<String>,
) -> impl IntoView {
    let close_popup = create_rw_signal(false);
    view! {
        <ActionTrackerPopup
            action=creation_action
            loading_message="Token creation in progress"
            modal=move |res| match res {
                Ok(_) => {
                    view! {
                        <CreateTokenSuccessPopup
                            img_url=img_url.get_untracked().clone()
                            token_name=token_name.get_untracked().clone()
                        />
                    }
                }
                Err(e) => {
                    view! {
                        <CreateTokenErrorPopup
                            close_popup=close_popup.write_only()
                            error=e
                            token_name=token_name.clone()
                        />
                    }
                }
            }

            close=close_popup
        />
    }
}

#[component]
fn TokenTransferSuccessPopup(
    #[prop(into)] token_name: String,
    amount: TokenBalance,
) -> impl IntoView {
    let amount_str = amount.humanize();
    view! {
        <SuccessPopup
            img=|| view! { <TokenConfettiSymbol class="w-8/12"/> }
            text=move || { format!("{amount_str} {token_name} Successfully sent") }

            previous_link="/wallet"
            previous_text="Back to wallet"
        />
    }
}

#[component]
fn TokenTransferErrorPopup(
    #[prop(into)] error: String,
    #[prop(into)] token_name: String,
    close_popup: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <ErrorPopup
            error
            header=move || {
                view! {
                    Failed to transfer
                    <span class="text-primary-600">{format!(" {token_name} ")}</span>
                    token
                }
            }

            previous_link="/wallet"
            previous_text="Back to wallet"
            close_popup
        />
    }
}

#[component]
pub fn TokenTransferPopup(
    transfer_action: Action<(), Result<TokenBalance, ServerFnError>>,
    #[prop(into)] token_name: MaybeSignal<String>,
) -> impl IntoView {
    let close_popup = create_rw_signal(false);

    view! {
        <ActionTrackerPopup
            action=transfer_action
            loading_message="Token transfer in progress"
            modal=move |res| match res {
                Ok(amount) => {
                    view! {
                        <TokenTransferSuccessPopup
                            token_name=token_name.get_untracked().clone()
                            amount
                        />
                    }
                }
                Err(e) => {
                    view! {
                        <TokenTransferErrorPopup
                            error=e.to_string()
                            token_name=token_name.get_untracked().clone()
                            close_popup=close_popup.write_only()
                        />
                    }
                }
            }

            close=close_popup
        />
    }
}
