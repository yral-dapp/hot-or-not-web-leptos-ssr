use leptos::*;
use leptos_dom::{html::Video, NodeRef};

use crate::{
    canister::utils::{bg_url, hls_stream_url, mp4_stream_url},
    js::videojs::{videojs, VideoJsPlayer},
};

#[component]
pub fn VideoPlayer(
    #[prop(optional)] node_ref: NodeRef<Video>,
    #[prop(into)] muted: WriteSignal<bool>,
    #[prop(into)] uid: MaybeSignal<Option<String>>,
    #[prop(optional)] autoplay: bool,
) -> impl IntoView {
    let uid = Signal::derive(uid);
    let view_bg_url = Signal::derive(move || uid().map(bg_url));
    let hls_url = Signal::derive(move || uid().map(hls_stream_url));
    let mp4_url = Signal::derive(move || uid().map(mp4_stream_url));
    let player = create_rw_signal(None::<VideoJsPlayer>);

    node_ref.on_load(move |v| {
        _ = v.on_mount(move |v| {
            let p = videojs(&v).unwrap();
            player.set(Some(p));
        });
    });

    on_cleanup(move || {
        let Some(p) = player.get() else {
            return;
        };
        _ = p.dispose();
    });

    view! {
        <label class="h-full w-full absolute top-0 left-0 grid grid-cols-1 justify-items-center items-center cursor-pointer z-[3]">
            <input
                on:change=move |_| muted.update(|m| *m = !*m)
                type="checkbox"
                value=""
                class="sr-only"
            />
            <div
                data-vjs-player
                style="background-color: transparent;"
                class="h-dvh max-h-dvh w-fit"
            >
                <video
                    node_ref=node_ref
                    class="video-js vjs-fill cursor-pointer"
                    poster=view_bg_url
                    loop
                    playsinline
                    autoplay=autoplay
                    disablepictureinpicture
                    disableremoteplayback
                    preload="auto"
                >
                    <source src=hls_url type="application/x-mpegURL"/>
                    <source src=mp4_url type="video/mp4"/>
                </video>
            </div>
        </label>
    }
}
