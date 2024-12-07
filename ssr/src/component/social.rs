use super::ic_symbol::IcSymbol;
use leptos::prelude::*;
use leptos_icons::*;

#[component]
fn FollowItem(#[prop(into)] href: String, #[prop(into)] icon: icondata::Icon) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            class="h-12 w-12 text-2xl rounded-full grid place-items-center aspect-square border border-primary-600"
        >
            <Icon icon />
        </a>
    }
}

macro_rules! social_button {
    ($name:ident, $icon:expr, $href:expr) => {
        #[component]
        pub fn $name() -> impl IntoView {
            view! {
                <FollowItem href=crate::consts::social::$href icon=$icon />
            }
        }
    };
}

social_button!(Telegram, icondata::TbBrandTelegram, TELEGRAM);
social_button!(Discord, icondata::BiDiscordAlt, DISCORD);
social_button!(Twitter, icondata::BiTwitter, TWITTER);
social_button!(IcWebsite, IcSymbol, IC_WEBSITE);
