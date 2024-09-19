use leptos::*;
use leptos_icons::*;

use crate::{component::overlay::*, utils::web::check_share_support};

#[component]
fn ShareProfileContent(
    share_link: String,
    message: String,
    close_popup: RwSignal<bool>,
) -> impl IntoView {
    let has_share_support = check_share_support();

    let share_link_social = share_link.clone();

    // Encode the message for URLs

    view! {
        <div class="flex flex-col gap-6 items-center p-6 w-full h-full bg-white rounded-lg shadow-lg">
            <div class="flex flex-col gap-2 items-center">
        <img class="w-16 h-16 md:w-20 md:h-20" src="/img/android-chrome-384x384.png" alt="YRAL Logo" />

        <span class="text-xl font-semibold text-center md:text-2xl">
            Share this app
        </span>
    </div>
            <Show when=move ||has_share_support.is_some()>
            <SocialShare message=message.clone() share_link=share_link_social.clone() />
            </Show>
          <div class="flex overflow-x-auto justify-center items-center px-10 mx-1 space-x-2 w-full rounded-xl border-2 border-neutral-700 h-[2.5rem] md:h-[5rem]">
        <span class="text-lg text-black md:text-xl truncate">
                {&share_link.clone()}
            </span>
            <button >
                <Icon class="w-6 h-6 text-black cursor-pointer" icon=icondata::BiCopyRegular />
            </button>
        </div>
        <button on:click=move|_| close_popup.set(true)
        class="py-4 w-3/4 text-lg text-center text-white rounded-full bg-primary-600"
    >
    Back
    </button>

        </div>
    }
}

#[component]
fn SocialShare(share_link: String, message: String) -> impl IntoView {
    let encoded_message = urlencoding::encode(&message);
    // let encoded_link = urlencoding::encode(&profile_link);

    // Facebook share URL using Dialog API
    let fb_url = format!(
        "http://www.facebook.com/share.php?u={}&title={}",
        share_link,
        urlencoding::encode("Check out this profile")
    );

    // WhatsApp share URL
    let whatsapp_url = format!("https://wa.me/?text={}", encoded_message);

    // Twitter share URL
    let twitter_url = format!("https://twitter.com/intent/tweet?text={}", encoded_message);

    // LinkedIn share URL
    let linkedin_url = format!(
        "https://www.linkedin.com/shareArticle?mini=true&url={}&title={}",
        share_link, encoded_message
    );

    // Functions to handle the share actions for each platform
    // let share_fb = move |_| {
    //     share_url(&fb_url);
    // };

    // let share_twitter = move |_| {
    //     share_url(&twitter_url);
    // };

    // let share_whatsapp = move |_| {
    //     share_url(&whatsapp_url);
    // };

    // let share_linkedin = move |_| {
    //     share_url(&linkedin_url);
    // };

    view! {
        <div class="flex gap-4">
                // Facebook button
                <a href=fb_url target="_blank">
                    <Icon
                        class="text-3xl md:text-4xl text-primary-600"
                        icon=icondata::BsFacebook
                    />
                </a>

                // Twitter button
                <a href=twitter_url target="_blank">
                    <Icon
                        class="text-3xl md:text-4xl text-primary-600"
                        icon=icondata::BsTwitterX
                    />
                </a>

                // WhatsApp button
                <a href=whatsapp_url target="_blank">
                    <Icon
                        class="text-3xl md:text-4xl text-primary-600"
                        icon=icondata::FaSquareWhatsappBrands
                    />
                </a>

                // LinkedIn button
                <a href=linkedin_url target="_blank">
                    <Icon
                        class="text-3xl md:text-4xl text-primary-600"
                        icon=icondata::TbBrandLinkedin
                    />
                </a>
            </div>
    }
}

#[component]
pub fn ShareProfilePopup(
    sharing_action: Action<(), Result<(), String>>,
    share_link: String,
    message: String,
) -> impl IntoView {
    let close_popup = create_rw_signal(false);

    view! {
         <PopupOverlay
                     loading_message=""

     action=sharing_action
             modal=move |_| {
                 view! {
                    <ShareProfileContent
                    share_link=share_link.clone()
                    message=message.clone()
                    close_popup
                    />
                 }
             }
    close=close_popup

         />
     }
}

#[component]
fn ProfileLoading() -> impl IntoView {
    view! {
        <div class="rounded-full animate-pulse basis-4/12 aspect-square overflow-clip bg-white/20"></div>
        <div class="flex flex-col gap-2 animate-pulse basis-8/12">
            <div class="w-full h-4 rounded-full bg-white/20"></div>
            <div class="w-full h-4 rounded-full bg-white/20"></div>
        </div>
    }
}
