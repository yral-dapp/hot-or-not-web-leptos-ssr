use leptos::*;
use leptos_icons::*;

#[component]
fn ShareProfileContent(
    #[prop(into)] profile_link: String,
    #[prop(into)] profile_image_url: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 items-center p-6 w-full h-full bg-white rounded-lg shadow-lg">
            <img
                class="object-cover w-20 h-20 rounded-full border-2 border-primary-600"
                style="height:10rem; width:10rem"
                src=profile_image_url
            />
            <span class="text-2xl font-bold text-center md:text-3xl">
                "Hey! Check out my YRAL profile 👇 {profile_link}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter"
            </span>
            <div class="flex gap-4">
                <a href={`https://www.facebook.com/sharer/sharer.php?u=${profile_link}`} target="_blank" class="p-2">
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::BsFacebook
                    />
                </a>
                <a href={`https://twitter.com/intent/tweet?url=${profile_link}&text=Check out my profile`} target="_blank" class="p-2">
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::BsTwitterX
                    />
                </a>
                <a href={`https://www.instagram.com/?url=${profile_link}`} target="_blank" class="p-2">
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::FaSquareInstagramBrands
                    />
                </a>
                <a href={`https://wa.me/?text=Check out my profile: ${profile_link}`} target="_blank" class="p-2">
                    <Icon
                        class="text-sm md:text-base text-primary-600"
                        icon=icondata::FaSquareWhatsappBrands
                    />
                </a>
            </div>
            <a
                href={profile_link}
                class="py-4 w-3/4 text-lg text-center text-white rounded-full bg-primary-600"
                target="_blank"
            >
                View Profile
            </a>
        </div>
    }
}

#[component]
pub fn ShareProfilePopup(
    #[prop(into)] profile_link: String,
    #[prop(into)] profile_image_url: String,
) -> impl IntoView {
    let close_popup = create_rw_signal(false);

    view! {
        <PopupOverlay
            action=Action::noop() // No action needed for this popup
            loading_message="Loading profile..."
            modal=move |_| {
                view! {
                    <ShareProfileContent
                        profile_link=profile_link.clone()
                        profile_image_url=profile_image_url.clone()
                    />
                }
            }
            close=close_popup
        />
    }
}

