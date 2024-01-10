use std::fmt::Display;

use crate::{consts::CF_STREAM_BASE, js::wasp::WaspHlsPlayerW};
use leptos::{
    html::{Img, Video},
    *,
};
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};

use super::VideoCtx;

fn bg_url(uid: impl Display) -> String {
    format!("{CF_STREAM_BASE}/{uid}/thumbnails/thumbnail.jpg")
}

fn stream_url(uid: impl Display) -> String {
    format!("{CF_STREAM_BASE}/{uid}/manifest/video.m3u8")
}

#[component]
pub fn BgView(uid: String, children: Children) -> impl IntoView {
    view! {
        <div
            class="bg-black bg-cover"
            style:background-image=move || format!("url({})", bg_url(&uid))
        >
            <div class="grid grid-cols-1 h-screen w-screen justify-items-center backdrop-blur-lg">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn HlsVideo(video_ref: NodeRef<Video>, allow_show: RwSignal<bool>) -> impl IntoView {
    let VideoCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();

    let current_uid =
        create_memo(move |_| with!(|video_queue| video_queue[current_idx()].uid.clone()));
    let wasp = create_rw_signal(None::<WaspHlsPlayerW>);
    let bg_url = move || bg_url(current_uid());

    create_effect(move |_| {
        let video = video_ref.get()?;
        log::debug!("initializing wasp player");
        let wasp_p = WaspHlsPlayerW::new(&video, None);
        video.set_muted(true);
        video.set_loop(true);
        wasp_p.add_event_listener("playerStateChange", move |state| match state.as_str() {
            "Loading" => allow_show.set(false),
            "Loaded" => allow_show.set(true),
            _ => (),
        });

        wasp.set(Some(wasp_p));

        Some(())
    });

    create_effect(move |_| {
        with!(|wasp| {
            let wasp = wasp.as_ref()?;
            let video = video_ref.get()?;
            wasp.stop();
            wasp.load(&stream_url(current_uid()));
            video.set_autoplay(true);
            video.set_poster(&bg_url());
            Some(())
        })
    });

    view! {
        <video
            _ref=video_ref
            class="object-contain h-screen muted autoplay"
            poster=bg_url
            loop
            muted
        ></video>
    }
}

#[component]
pub fn ThumbView(idx: usize) -> impl IntoView {
    let container_ref = create_node_ref::<Img>();
    let VideoCtx {
        video_queue,
        trigger_fetch,
        current_idx,
        ..
    } = expect_context();

    let uid = create_memo(move |_| with!(|video_queue| video_queue[idx].uid.clone()));
    let view_bg_url = move || bg_url(uid());

    use_intersection_observer_with_options(
        container_ref,
        move |entry, _| {
            let Some(visible) = entry
                .into_iter()
                .find(|entry| entry.is_intersecting() && entry.intersection_ratio() == 1.0)
            else {
                return;
            };
            let rect = visible.bounding_client_rect();
            // TODO: confirm this in different screens and browsers
            // this prevents an initial back and forth between the first and second video
            if rect.y() == rect.height() || idx == current_idx.get_untracked() {
                return;
            }

            // fetch new videos
            if idx == 14 || (idx > 14 && idx % 8 == 0) {
                log::debug!("trigger rerender");
                trigger_fetch.dispatch(());
            }
            current_idx.set(idx);
        },
        UseIntersectionObserverOptions::default().thresholds(vec![1.0]),
    );

    view! { <img class="object-contain h-screen" src=view_bg_url _ref=container_ref/> }
}
