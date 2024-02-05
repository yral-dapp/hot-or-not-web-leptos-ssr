use crate::canister::utils::{bg_url, mp4_url};
use leptos::{html::Video, *};
use leptos_use::{
    use_document, use_intersection_observer_with_options, UseIntersectionObserverOptions,
};

use super::PostViewCtx;

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
pub fn VideoView(idx: usize, muted: RwSignal<bool>) -> impl IntoView {
    let container_ref = create_node_ref::<Video>();
    let PostViewCtx {
        video_queue,
        fetch_cursor,
        current_idx,
        ..
    } = expect_context();

    let uid =
        create_memo(move |_| with!(|video_queue| video_queue.get(idx).map(|q| q.uid.clone())));
    let view_bg_url = move || uid().map(bg_url);
    let view_video_url = move || uid().map(mp4_url);

    use_intersection_observer_with_options(
        container_ref,
        move |entry, _| {
            let Some(visible) = entry
                .into_iter()
                .find(|entry| entry.is_intersecting() && entry.intersection_ratio() >= 0.8)
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
            if video_queue.with_untracked(|q| q.len()).saturating_sub(idx) == 10 {
                log::debug!("trigger rerender");
                fetch_cursor.update(|c| c.advance());
            }
            current_idx.set(idx);
        },
        UseIntersectionObserverOptions::default()
            .thresholds(vec![1.0])
            .root(use_document().as_ref().and_then(|d| d.body())),
    );

    // Handles autoplay
    create_effect(move |_| {
        let vid = container_ref().unwrap();
        if idx != current_idx() {
            _ = vid.pause();
            return;
        }
        vid.scroll_into_view();
        vid.set_autoplay(true);
        _ = vid.play();
    });

    // Handles mute/unmute
    create_effect(move |_| {
        let vid = container_ref().unwrap();
        vid.set_muted(muted());
    });

    create_effect(move |_| {
        let vid = container_ref().unwrap();
        // the attributes in DOM don't seem to be working
        vid.set_muted(muted.get_untracked());
        vid.set_loop(true);
    });

    view! {
        <video
            on:click=move |_| muted.update(|m| *m = !*m)
            _ref=container_ref
            class="object-contain h-full cursor-pointer"
            poster=view_bg_url
            src=view_video_url
            loop
            muted
            preload="auto"
        ></video>
    }
}
