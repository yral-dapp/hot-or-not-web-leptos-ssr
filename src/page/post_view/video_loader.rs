use crate::{
    canister::utils::{bg_url, stream_url},
    js::wasp::WaspHlsPlayerW,
};
use leptos::{
    html::{Img, Video},
    *,
};
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};

use super::VideoCtx;

#[component]
pub fn BgView(uid: String, children: Children) -> impl IntoView {
    view! {
        <div
            class="bg-black bg-cover h-full"
            style:background-image=move || format!("url({})", bg_url(&uid))
        >
            <div class="grid grid-cols-1 h-full w-full justify-items-center backdrop-blur-lg">
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

    let current_uid = create_memo(move |_| {
        with!(|video_queue| video_queue.get(current_idx()).map(|q| q.uid.clone()))
    });
    let wasp = create_rw_signal(None::<WaspHlsPlayerW>);
    let bg_url = move || current_uid().map(bg_url);

    create_effect(move |_| {
        let video = video_ref.get()?;
        let video = video.classes("object-contain h-full");
        log::debug!("initializing wasp player");
        let wasp_p = WaspHlsPlayerW::new(&video, None);
        video.set_muted(true);
        video.set_loop(true);
        video.set_autoplay(true);
        wasp_p.add_event_listener("playerStateChange", move |state| match state.as_str() {
            "Loading" => allow_show.set(false),
            "Loaded" => {
                allow_show.set(true);
                if video.paused() {
                    _ = video.play();
                }
            }
            _ => (),
        });

        wasp.set(Some(wasp_p));

        Some(())
    });

    create_effect(move |_| {
        with!(|wasp| {
            let wasp = wasp.as_ref()?;
            let video = video_ref.get()?;
            wasp.load(&stream_url(current_uid()?));
            video.set_poster(&bg_url()?);
            Some(())
        })
    });

    view! { <video _ref=video_ref class="object-contain autoplay h-full" poster=bg_url loop muted></video> }
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

    let uid =
        create_memo(move |_| with!(|video_queue| video_queue.get(idx).map(|q| q.uid.clone())));
    let view_bg_url = move || uid().map(bg_url);

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
                trigger_fetch.refetch();
            }
            current_idx.set(idx);
        },
        UseIntersectionObserverOptions::default().thresholds(vec![1.0]),
    );

    view! { <img class="object-contain h-full" src=view_bg_url _ref=container_ref/> }
}
