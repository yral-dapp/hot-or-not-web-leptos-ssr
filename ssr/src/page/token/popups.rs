use crate::{
    component::{overlay::PopupOverlay, token_confetti_symbol::TokenConfettiSymbol},
    page::token::create::CreateTokenCtx,
    state::canisters::auth_canisters_store,
    utils::token::TokenBalance,
};
use leptos::*;
use leptos_icons::*;

#[component]
fn SuccessPopup<ImgIV: IntoView, Img: Fn() -> ImgIV, TxtIV: IntoView, Txt: Fn() -> TxtIV>(
    img: Img,
    text: Txt,
    #[prop(into)] previous_link: MaybeSignal<String>,
    #[prop(into)] previous_text: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 items-center w-full h-full">
            {img()} <span class="text-2xl font-bold text-center md:text-3xl">{text()}</span>
            <a
                href=previous_link
                class="py-4 w-3/4 text-lg text-center text-white rounded-full bg-primary-600"
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
    let cans = auth_canisters_store();
    let profile_url = Signal::derive(move || {
        let Some(cans) = cans() else {
            return "/menu".into();
        };
        let profile_id = cans.user_principal();
        format!("/your-profile/{profile_id}?tab=tokens")
    });
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
        <div class="flex flex-col gap-6 items-center w-full h-full">
            <div class="flex flex-row justify-center items-center p-3 text-2xl text-orange-400 bg-amber-100 rounded-full md:text-3xl">
                <Icon icon=icondata::BsExclamationTriangle/>
            </div>
            <span class="text-2xl font-bold text-center md:text-3xl">{header()}</span>
            <textarea
                prop:value=error
                disabled
                rows=3
                class="p-2 w-full text-xs text-red-500 resize-none md:w-2/3 md:text-sm bg-black/10"
            ></textarea>
            <button
                on:click=move |_| close_popup.set(true)
                class="py-3 w-full text-lg text-center text-white rounded-full md:text-xl bg-primary-600"
            >
                Retry
            </button>
            <a
                href=previous_link
                class="py-3 w-full text-lg text-center text-black bg-white rounded-full border border-black md:text-xl"
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
    let cans = auth_canisters_store();
    let profile_url = Signal::derive(move || {
        let Some(cans) = cans() else {
            return "/menu".into();
        };
        let profile_id = cans.user_principal();
        format!("/your-profile/{profile_id}?tab=tokens")
    });

    view! {
        <ErrorPopup
            error
            header=move || {
                let token_name = token_name.clone();
                view! {
                    Token
                    <span class="text-primary-600">{move || format!(" {} ", token_name.with(|t| t.clone()))}</span>
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
        <PopupOverlay
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
        <PopupOverlay
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
#[component]
fn ShareProfileContent() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 items-center p-6 w-full h-full bg-white rounded-lg shadow-lg">
            // <img
            //     class="object-cover w-20 h-20 rounded-full border-2 border-primary-600"
            //     style="height:10rem; width:10rem"
            //     src=profile_image_url
            // />
            <span class="text-2xl font-bold text-center md:text-3xl">
                "Hey! Check out my YRAL profile 👇 {profile_link}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter"
            </span>
            <div class="flex gap-4">
                <a href="/" >
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::BsFacebook
                    />
                </a>
                <a href="/">
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::BsTwitterX
                    />
                </a>
                <a href="/">
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::FaSquareInstagramBrands
                    />
                </a>
                <a href="/">

                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::FaSquareWhatsappBrands
                    />
                </a>
            </div>
            <a
                href="/"
                class="py-4 w-3/4 text-lg text-center text-white rounded-full bg-primary-600"
                target="_blank"
            >
                View Profile
            </a>
        </div>
    }
}

#[component]
pub fn ShareProfilePopup(sharing_action: Action<(), Result<(), String>>) -> impl IntoView {
    let close_popup = create_rw_signal(false);

    view! {
         <PopupOverlay
                     loading_message="Token creation in progress"

     action=sharing_action
             modal=move |_| {
                 view! {
                     <ShareProfileContent


                     />
                 }
             }
    close=close_popup

         />
     }
}
