use leptos::*;
use leptos_icons::*;
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};

use crate::page::post_view::video_loader::{BgView, VideoViewForQueue};

use crate::state::audio_state::AudioState;
use crate::utils::posts::PostDetails;

#[component]
pub fn MuteIconOverlay(show_mute_icon: RwSignal<bool>) -> impl IntoView {
    view! {
        <Show when=show_mute_icon>
            <button
                class="fixed top-1/2 left-1/2 z-20 cursor-pointer pointer-events-none"
                on:click=move |_| AudioState::toggle_mute()
            >
                <Icon
                    class="text-white/80 animate-ping text-4xl"
                    icon=icondata::BiVolumeMuteSolid
                />
            </button>
        </Show>
    }
}

#[component]
pub fn ScrollingPostViewMLFeed<F: Fn() -> V + Clone + 'static, V>(
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    #[prop(optional)] fetch_next_videos: Option<F>,
    recovering_state: RwSignal<bool>,
    queue_end: RwSignal<bool>,
    #[prop(optional, into)] overlay: Option<ViewFn>,
    threshold_trigger_fetch: usize,
) -> impl IntoView {
    let AudioState {
        muted,
        show_mute_icon,
        ..
    } = AudioState::get();

    let scroll_root: NodeRef<html::Div> = create_node_ref();

    // let update_feed_action = create_action(move |_| async move {
    //     // // pop from priority_q and push to video_queue until video_queue.with_untracked(|q| q.len()).saturating_sub(current_idx.get_untracked()) > 20 or priority_q is empty
    //     update!(move |video_queue, priority_q| {
    //         let mut cnt = 0;
    //         leptos::logging::log!("priority_q length: {}", priority_q.len());
    //         while let Some(next) = priority_q.pop() {
    //             video_queue.push(next);
    //             cnt += 1;
    //             if cnt >= 15 {
    //                 break;
    //             }
    //         }
    //     });
    // });

    // let update_feed = use_debounce_fn(
    //     move || {
    //         if !update_feed_action.pending().get_untracked() {
    //             update_feed_action.dispatch(())
    //         }
    //     },
    //     200.0,
    // );

    let var_name = view! {
        <div class="h-full w-full overflow-hidden overflow-y-auto">
            <div
                _ref=scroll_root
                class="snap-mandatory snap-y overflow-y-scroll h-dvh w-dvw bg-black"
                style:scroll-snap-points-y="repeat(100vh)"
            >

                {overlay.map(|o| o.run())}

                <For
                    each=move || video_queue().into_iter().enumerate()
                    key=move |(_, details)| (details.canister_id, details.post_id)
                    children=move |(queue_idx, _details)| {
                        let container_ref = create_node_ref::<html::Div>();
                        let next_videos = fetch_next_videos.clone();
                        // let update_feed = update_feed.clone();
                        use_intersection_observer_with_options(
                            container_ref,
                            move |entry, _| {
                                let Some(visible) = entry.first().filter(|e| e.is_intersecting())
                                else {
                                    return;
                                };
                                let rect = visible.bounding_client_rect();
                                if rect.y() == rect.height()
                                    || queue_idx == current_idx.get_untracked()
                                {
                                    return;
                                }
                                // if video_queue.with_untracked(|q| q.len()).saturating_sub(queue_idx)
                                //     <= 20 {
                                //         // update_feed_action.dispatch(());
                                //         update!(move |video_queue, priority_q| {
                                //             let mut cnt = 0;
                                //             leptos::logging::log!("priority_q length: {}", priority_q.len());
                                //             while let Some(next) = priority_q.pop() {
                                //                 video_queue.push(next);
                                //                 cnt += 1;
                                //                 if cnt >= 15 {
                                //                     break;
                                //                 }
                                //             }
                                //         });
                                // }
                                // if priority_q.get_untracked().len() <= 40
                                // {
                                //     next_videos.as_ref().map(|nv| { nv() });
                                // }

                                if video_queue.with_untracked(|q| q.len()).saturating_sub(queue_idx)
                                <= threshold_trigger_fetch {
                                        next_videos.as_ref().map(|nv| { nv() });
                                    }

                                current_idx.set(queue_idx);
                            },
                            UseIntersectionObserverOptions::default()
                                .thresholds(vec![0.83])
                                .root(Some(scroll_root)),
                        );
                        create_effect(move |_| {
                            let Some(container) = container_ref() else {
                                return;
                            };
                            if current_idx() == queue_idx && recovering_state.get_untracked() {
                                container.scroll_into_view();
                                recovering_state.set(false);
                            }
                        });
                        let show_video = create_memo(move |_| {
                            queue_idx.abs_diff(current_idx()) <= 20
                        });
                        view! {
                            <div _ref=container_ref class="snap-always snap-end w-full h-full">
                                <Show when=show_video>
                                    <BgView video_queue current_idx idx=queue_idx>
                                        <VideoViewForQueue video_queue current_idx idx=queue_idx muted />
                                    </BgView>
                                </Show>
                            </div>
                        }
                    }
                />

                <Show when=queue_end>
                    <div class="h-full w-full relative top-0 left-0 bg-inherit z-[21] flex snap-always snap-end justify-center items-center text-xl text-white/80">
                        <span>You have reached the end!</span>
                    </div>
                </Show>

                <MuteIconOverlay show_mute_icon/>
            </div>
        </div>
    };
    var_name
}

#[component]
pub fn ScrollingPostView<F: Fn() -> V + Clone + 'static, V>(
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    #[prop(optional)] fetch_next_videos: Option<F>,
    recovering_state: RwSignal<bool>,
    queue_end: RwSignal<bool>,
    #[prop(optional, into)] overlay: Option<ViewFn>,
    threshold_trigger_fetch: usize,
) -> impl IntoView {
    let AudioState {
        muted,
        show_mute_icon,
        ..
    } = AudioState::get();

    let scroll_root: NodeRef<html::Div> = create_node_ref();

    let var_name = view! {
        <div class="h-full w-full overflow-hidden overflow-y-auto">
            <div
                _ref=scroll_root
                class="snap-mandatory snap-y overflow-y-scroll h-dvh w-dvw bg-black"
                style:scroll-snap-points-y="repeat(100vh)"
            >

                {overlay.map(|o| o.run())}

                <For
                    each=move || video_queue().into_iter().enumerate()
                    key=move |(_, details)| (details.canister_id, details.post_id)
                    children=move |(queue_idx, _details)| {
                        let container_ref = create_node_ref::<html::Div>();
                        let next_videos = fetch_next_videos.clone();
                        use_intersection_observer_with_options(
                            container_ref,
                            move |entry, _| {
                                let Some(visible) = entry.first().filter(|e| e.is_intersecting())
                                else {
                                    return;
                                };
                                let rect = visible.bounding_client_rect();
                                if rect.y() == rect.height()
                                    || queue_idx == current_idx.get_untracked()
                                {
                                    return;
                                }
                                if video_queue.with_untracked(|q| q.len()).saturating_sub(queue_idx)
                                <= threshold_trigger_fetch
                                {
                                    next_videos.as_ref().map(|nv| { nv() });
                                }
                                current_idx.set(queue_idx);
                            },
                            UseIntersectionObserverOptions::default()
                                .thresholds(vec![0.83])
                                .root(Some(scroll_root)),
                        );
                        create_effect(move |_| {
                            let Some(container) = container_ref() else {
                                return;
                            };
                            if current_idx() == queue_idx && recovering_state.get_untracked() {
                                container.scroll_into_view();
                                recovering_state.set(false);
                            }
                        });
                        let show_video = create_memo(move |_| {
                            queue_idx.abs_diff(current_idx()) <= 20
                        });
                        view! {
                            <div _ref=container_ref class="snap-always snap-end w-full h-full">
                                <Show when=show_video>
                                    <BgView video_queue current_idx idx=queue_idx>
                                        <VideoViewForQueue video_queue current_idx idx=queue_idx muted />
                                    </BgView>
                                </Show>
                            </div>
                        }
                    }
                />

                <Show when=queue_end>
                    <div class="h-full w-full relative top-0 left-0 bg-inherit z-[21] flex snap-always snap-end justify-center items-center text-xl text-white/80">
                        <span>You have reached the end!</span>
                    </div>
                </Show>

                <MuteIconOverlay show_mute_icon/>
            </div>
        </div>
    };
    var_name
}
