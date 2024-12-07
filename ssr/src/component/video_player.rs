use leptos::{html::Video, prelude::*};

use crate::state::audio_state::AudioState;

#[component]
pub fn VideoPlayer(
    #[prop(optional)] node_ref: NodeRef<Video>,
    #[prop(into)] view_bg_url: Signal<Option<String>>,
    #[prop(into)] view_video_url: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <label class="h-full w-full absolute top-0 left-0 grid grid-cols-1 justify-items-center items-center cursor-pointer z-[3]">
            <input
                on:change=move |_| AudioState::toggle_mute()
                type="checkbox"
                value=""
                class="sr-only"
            />
            <video
                node_ref=node_ref
                class="object-contain h-dvh max-h-dvh cursor-pointer"
                poster=view_bg_url
                src=view_video_url
                loop
                muted
                playsinline
                disablepictureinpicture
                disableremoteplayback
                preload="auto"
            ></video>
        </label>
    }
}
