use leptos::prelude::*;

use crate::component::title::TitleText;

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
        <div class="flex flex-col w-screen h-screen items-center bg-black pt-4 pb-12 px-8 gap-16">
            <TitleText>
                <span class="text-lg font-bold text-white">About Us</span>
            </TitleText>
            <div class="flex flex-col w-full gap-14 text-lg">
                <span class="text-primary-600 font-bold">Reinventing Short Video Social Media</span>
                <div class="flex flex-col w-full gap-4 text-white">
                    <span class="font-bold">Think TikTok on Blockchain</span>
                    <span class="font-light text-md">
                        Hot or Not is a short-video social media platform on blockchain, which
                        not only provides the fun and excitement of social media but also
                        enables the users to earn rewards for creating content, engaging with
                        it or sharing it with their friends and family!
                    </span>
                </div>
            </div>
        </div>
    }
}
