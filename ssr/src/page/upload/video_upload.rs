use super::{
    cf_upload::{get_upload_info, get_video_status, publish_video, upload_video_stream},
    UploadParams,
};
use crate::{
    component::modal::Modal,
    state::canisters::{auth_canisters_store, authenticated_canisters},
    try_or_redirect_opt,
    utils::{
        event_streaming::events::{
            VideoUploadSuccessful, VideoUploadUnsuccessful, VideoUploadVideoSelected,
        },
        route::go_to_root,
        web::FileWithUrl,
    },
};
use futures::StreamExt;
use gloo::timers::future::IntervalStream;
use ic_agent::Identity;
use leptos::{
    ev::durationchange,
    html::{Input, Video},
    prelude::*,
};
use leptos_icons::*;
use leptos_use::use_event_listener;
use web_time::SystemTime;
use yral_canisters_common::Canisters;

#[component]
pub fn DropBox() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-self-center justify-center w-full border-2 border-dashed rounded-lg cursor-pointer border-gray-600 hover:bg-gray-600 aspect-[3/4] lg:aspect-[5/4]">
            <Icon class="w-10 h-10 mb-4 text-gray-400" icon=icondata::BiCloudUploadRegular />
            <p class="text-center mb-2 mx-2 text-sm text-gray-400">
                <span class="font-semibold">Click to upload</span>
                or drag and drop
            </p>
            <p class="text-xs text-gray-400">Video File (Max 60s)</p>
        </div>
    }
}

#[component]
pub fn PreVideoUpload(file_blob: WriteSignal<Option<FileWithUrl>, LocalStorage>) -> impl IntoView {
    let file_ref = NodeRef::<Input>::new();
    let file = RwSignal::new_local(None::<FileWithUrl>);
    let video_ref = NodeRef::<Video>::new();
    let modal_show = RwSignal::new(false);
    let canister_store = auth_canisters_store();

    #[cfg(feature = "hydrate")]
    {
        use leptos::ev::change;
        _ = use_event_listener(file_ref, change, move |ev| {
            use wasm_bindgen::JsCast;
            use web_sys::HtmlInputElement;
            ev.target().and_then(move |target| {
                let input: &HtmlInputElement = target.dyn_ref()?;
                let inp_file = input.files()?.get(0)?;
                file.set(Some(FileWithUrl::new(inp_file.into())));

                VideoUploadVideoSelected.send_event(canister_store);
                Some(())
            });
        });
    }

    _ = use_event_listener(video_ref, durationchange, move |_| {
        let duration = video_ref
            .get_untracked()
            .map(|v| v.duration())
            .unwrap_or_default();
        let Some(vid_file) = file.get_untracked() else {
            return;
        };
        if duration <= 60.0 || duration.is_nan() {
            modal_show.set(false);
            file_blob.set(Some(vid_file));
            return;
        }

        file.set(None);
        file_blob.set(None);
        modal_show.set(true);
        if let Some(f) = file_ref.get_untracked() {
            f.set_value("");
        }
    });

    view! {
        <div class="flex items-center self-center justify-center w-3/4 mb-8 lg:mb-0 lg:pb-12 lg:w-1/2 lg:max-h-full lg:px-8">
            <label
                for="dropzone-file"
                class="flex justify-start flex-col h-full w-full cursor-pointer"
            >
                <Show when=move || { file.with(|f| f.is_none()) }>
                    <DropBox />
                </Show>
                <video
                    node_ref=video_ref
                    class="object-contain w-full"
                    playsinline
                    muted
                    autoplay
                    loop
                    oncanplay="this.muted=true"
                    src=move || file.with(|f| f.as_ref().map(|f| f.url.to_string()))
                    style:display=move || {
                        file.with(|f| f.as_ref().map(|_| "block").unwrap_or("none"))
                    }
                ></video>
                <input
                    on:click=move |_| modal_show.set(true)
                    id="dropzone-file"
                    node_ref=file_ref
                    type="file"
                    accept="video/*"
                    class="hidden w-0 h-0"
                />
            </label>
        </div>
        <Modal show=modal_show>
            <span class="text-lg md:text-xl text-white h-full items-center py-10 text-center w-full flex flex-col justify-center">
                Please ensure that the video is shorter than 60 seconds
            </span>
        </Modal>
    }
}

#[component]
pub fn ProgressItem(
    #[prop(into)] initial_text: String,
    #[prop(into)] done_text: String,
    #[prop(into)] loading: Signal<bool>,
) -> impl IntoView {
    view! {
        <Show
            when=loading
            fallback=move || {
                view! {
                    <Icon class="w-10 h-10 text-green-600" icon=icondata::BsCheckCircleFill />
                    <span class="text-white text-lg font-semibold">{done_text.clone()}</span>
                }
            }
        >

            <Icon class="w-10 h-10 text-primary-600 animate-spin" icon=icondata::CgSpinnerTwo />
            <span class="text-white text-lg font-semibold">{initial_text.clone()}</span>
        </Show>
    }
}

#[component]
pub fn VideoUploader(params: UploadParams) -> impl IntoView {
    let file_blob = params.file_blob;
    let hashtags = params.hashtags;
    let description = params.description;

    let uploading = RwSignal::new(true);
    let processing = RwSignal::new(true);
    let publishing = RwSignal::new(true);
    let video_url = StoredValue::new_local(file_blob.url);
    let file_blob = file_blob.file.clone();

    let up_hashtags = hashtags.clone();
    let hashtags_len = hashtags.len();
    let is_nsfw = params.is_nsfw;
    let enable_hot_or_not = params.enable_hot_or_not;
    let canister_store = auth_canisters_store();

    let up_desc = description.clone();

    let upload_action = LocalResource::new(move || {
        let cans = canister_store.get();
        let hashtags = up_hashtags.clone();
        let description = up_desc.clone();
        let file_blob = file_blob.clone();
        async move {
            let cans = cans?;
            let creator_principal = cans.identity().sender().unwrap();
            let time_ms = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            // TODO: authenticated call
            let res = get_upload_info(
                creator_principal,
                hashtags,
                description,
                time_ms.to_string(),
            )
            .await;

            if res.is_err() {
                let e = res.as_ref().err().unwrap().to_string();
                VideoUploadUnsuccessful.send_event(
                    e,
                    hashtags_len,
                    is_nsfw,
                    enable_hot_or_not,
                    canister_store,
                );
            }

            let upload_info = try_or_redirect_opt!(res);

            let res = upload_video_stream(&upload_info, &file_blob).await;

            if res.is_err() {
                let e = res.as_ref().err().unwrap().to_string();
                VideoUploadUnsuccessful.send_event(
                    e,
                    hashtags_len,
                    is_nsfw,
                    enable_hot_or_not,
                    canister_store,
                );
            }

            try_or_redirect_opt!(res);

            uploading.set(false);

            let mut check_status = IntervalStream::new(4000);
            while (check_status.next().await).is_some() {
                let uid = upload_info.uid.clone();
                let res = get_video_status(uid).await;

                if res.is_err() {
                    let e = res.as_ref().err().unwrap().to_string();
                    VideoUploadUnsuccessful.send_event(
                        e,
                        hashtags_len,
                        is_nsfw,
                        enable_hot_or_not,
                        canister_store,
                    );
                }

                let status = try_or_redirect_opt!(res);
                if status == "ready" {
                    break;
                }
            }
            processing.set(false);

            Some(upload_info.uid)
        }
    });

    let publish_action: Action<_, _, LocalStorage> =
        Action::new_unsync(move |(canisters, uid): &(Canisters<true>, String)| {
            let canisters = canisters.clone();
            let hashtags = hashtags.clone();
            let hashtags_len = hashtags.len();
            let description = description.clone();
            let uid = uid.clone();
            async move {
                let res = publish_video(
                    canisters,
                    hashtags,
                    description,
                    uid.clone(),
                    params.enable_hot_or_not,
                    params.is_nsfw,
                )
                .await;

                if res.is_err() {
                    let e = res.as_ref().err().unwrap().to_string();
                    VideoUploadUnsuccessful.send_event(
                        e,
                        hashtags_len,
                        is_nsfw,
                        enable_hot_or_not,
                        canister_store,
                    );
                }

                try_or_redirect_opt!(res);

                publishing.set(false);

                let post_id = res.unwrap();
                VideoUploadSuccessful.send_event(
                    uid,
                    hashtags_len,
                    is_nsfw,
                    enable_hot_or_not,
                    post_id,
                    canister_store,
                );

                Some(())
            }
        });
    let cans_res = authenticated_canisters();

    view! {
        <div class="flex flex-col justify-start self-center w-3/4 mb-8 lg:mb-0 lg:pb-12 lg:max-h-full lg:w-1/2 basis-full lg:basis-5/12">
            <video
                class="object-contain w-full"
                playsinline
                muted
                autoplay
                loop
                oncanplay="this.muted=true"
                src=move || video_url.get_value().to_string()
            ></video>
        </div>
        <div class="flex flex-col basis-full lg:basis-7/12 gap-4 px-4">
            <div class="flex flex-row gap-4">
                <ProgressItem initial_text="Uploading" done_text="Uploaded" loading=uploading />
            </div>
            <div class="flex flex-row gap-4">
                <ProgressItem initial_text="Processing" done_text="Processed" loading=processing />
            </div>
            <div class="flex flex-row gap-4">
                <ProgressItem initial_text="Publishing" done_text="Published" loading=publishing />
                <Suspense>
                    {move || Suspend::new(async move {
                        let uid = upload_action.await?;
                        let cans_wire = cans_res.await.ok()?;
                        let canisters = Canisters::from_wire(cans_wire, expect_context()).ok()?;
                        publish_action.dispatch((canisters, uid));
                        Some(())
                    })}

                </Suspense>
            </div>
            <button
                on:click=|_| go_to_root()
                disabled=publishing
                class="py-3 w-5/6 md:w-4/6 my-8 self-center disabled:bg-primary-400 disabled:text-white/80 bg-green-600 rounded-full font-bold text-md md:text-lg lg:text-xl"
            >
                Continue Browsing
            </button>
        </div>
    }
}
