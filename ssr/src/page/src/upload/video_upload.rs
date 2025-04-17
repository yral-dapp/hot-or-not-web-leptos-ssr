use super::UploadParams;
use auth::delegate_short_lived_identity;
use component::modal::Modal;
use gloo::net::http::Request;
use leptos::{
    ev::durationchange,
    html::{Input, Video},
    prelude::*,
};
use leptos_icons::*;
use leptos_use::use_event_listener;
use serde::{Deserialize, Serialize};
use serde_json::json;
use state::canisters::authenticated_canisters;
use utils::{
    event_streaming::events::{
        auth_canisters_store, VideoUploadSuccessful, VideoUploadUnsuccessful,
        VideoUploadVideoSelected,
    },
    route::go_to_root,
    try_or_redirect_opt,
    web::FileWithUrl,
};
use leptos::web_sys::{Blob, FormData};
use yral_canisters_common::Canisters;

#[component]
pub fn DropBox() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-self-center justify-center w-full border-2 border-dashed rounded-lg cursor-pointer border-gray-600 hover:bg-gray-600 aspect-[3/4] lg:aspect-[5/4]">
            <Icon attr:class="w-10 h-10 mb-4 text-gray-400" icon=icondata::BiCloudUploadRegular />
            <p class="text-center mb-2 mx-2 text-sm text-gray-400">
                <span class="font-semibold">Click to upload</span>
                or drag and drop
            </p>
            <p class="text-xs text-gray-400">Video File (Max 60s)</p>
        </div>
    }
}

#[component]
pub fn PreVideoUpload(
    file_blob: RwSignal<Option<FileWithUrl>, LocalStorage>,
    uid: RwSignal<String, LocalStorage>,
) -> impl IntoView {
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

    let canister_store = auth_canisters_store();

    let upload_action: Action<(), (), LocalStorage> = Action::new_local(move |_| async move {
        let upload_base_url = "https://yral-upload-video.go-bazzinga.workers.dev";

        let message = upload_video_part(
            upload_base_url,
            "file",
            file_blob.get_untracked().unwrap().file.as_ref(),
        )
        .await
        .inspect_err(|e| {
            VideoUploadUnsuccessful.send_event(e.to_string(), 0, false, true, canister_store);
        })
        .unwrap();

        uid.set(message.data.unwrap().uid.unwrap());
    });

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
            upload_action.dispatch(());
            return;
        }

        modal_show.set(true);
        file.set(None);
        file_blob.set(None);
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
                <Show when=move || { file.with(| file | file.is_none()) }>
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
                    src=move || file.with(| file | file.as_ref().map(| f | f.url.to_string()))
                    style:display=move || {
                        file.with(| file | file.as_ref().map(| _ | "block").unwrap_or("none"))
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
                    <Icon attr:class="w-10 h-10 text-green-600" icon=icondata::BsCheckCircleFill />
                    <span class="text-white text-lg font-semibold">{done_text.clone()}</span>
                }
            }
        >

            <Icon attr:class="w-10 h-10 text-primary-600 animate-spin" icon=icondata::CgSpinnerTwo />
            <span class="text-white text-lg font-semibold">{initial_text.clone()}</span>
        </Show>
    }
}
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub message: Option<String>,
    pub success: Option<bool>,
    pub data: Option<Data>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename = "scheduledDeletion")]
    pub scheduled_deletion: Option<String>,
    pub uid: Option<String>,
    #[serde(rename = "uploadURL")]
    pub upload_url: Option<String>,
    pub watermark: Option<Watermark>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Watermark {
    pub created: Option<String>,
    #[serde(rename = "downloadedFrom")]
    pub downloaded_from: Option<String>,
    pub height: Option<f64>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub padding: Option<f64>,
    pub position: Option<String>,
    pub scale: Option<f64>,
    pub size: Option<f64>,
    pub uid: Option<String>,
}
#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct VideoMetadata {
    pub title: String,
    pub description: String,
    pub tags: String,
}

#[derive(Serialize, Debug)]
pub struct SerializablePostDetailsFromFrontend {
    pub is_nsfw: bool,
    pub hashtags: Vec<String>,
    pub description: String,
    pub video_uid: String,
    pub creator_consent_for_inclusion_in_hot_or_not: bool,
}

async fn upload_video_part(
    upload_base_url: &str,
    form_field_name: &str,
    file_blob: &Blob,
) -> Result<Message, ServerFnError> {
    let get_url_endpoint = format!("{}/get_upload_url", upload_base_url);
    let response = Request::get(&get_url_endpoint).send().await?;
    if !response.ok() {
        return Err(ServerFnError::new(format!(
            "Failed to get upload URL: status {}",
            response.status()
        )));
    }
    let response_text = response.text().await?;
    let upload_message: Message = serde_json::from_str(&response_text)
        .map_err(|e| ServerFnError::new(format!("Failed to parse upload URL response: {}", e)))?;

    let upload_url = upload_message
        .data
        .clone()
        .and_then(|d| d.upload_url)
        .ok_or_else(|| ServerFnError::new("Upload URL not found in response".to_string()))?;

    let form = FormData::new().map_err(|js_value| {
        ServerFnError::new(format!("Failed to create FormData: {:?}", js_value))
    })?;
    form.append_with_blob(form_field_name, file_blob)
        .map_err(|js_value| {
            ServerFnError::new(format!("Failed to append blob to FormData: {:?}", js_value))
        })?;

    let upload_response = Request::post(&upload_url).body(form)?.send().await?;

    if !upload_response.ok() {
        return Err(ServerFnError::new(format!(
            "Upload request failed: status {} {}",
            upload_response.status(),
            upload_response.status_text()
        )));
    }

    Ok(upload_message)
}

#[component]
pub fn VideoUploader(params: UploadParams, uid: RwSignal<String, LocalStorage>) -> impl IntoView {
    let file_blob = params.file_blob;
    let hashtags = params.hashtags;
    let description = params.description;

    let uploading = RwSignal::new(true);
    let processing = RwSignal::new(true);
    let publishing = RwSignal::new(true);
    let video_url = StoredValue::new_local(file_blob.url);

    let is_nsfw = params.is_nsfw;
    let enable_hot_or_not = params.enable_hot_or_not;
    let canister_store = auth_canisters_store();

    let publish_action: Action<_, _, LocalStorage> =
        Action::new_unsync(move |canisters: &Canisters<true>| {
            let canisters = canisters.clone();
            let hashtags = hashtags.clone();
            let hashtags_len = hashtags.len();
            let description = description.clone();
            let uid = uid.get_untracked();
            async move {
                let upload_base_url = "https://yral-upload-video.go-bazzinga.workers.dev";
                let id = canisters.identity();
                let delegated_identity = delegate_short_lived_identity(id);
                let res: std::result::Result<gloo::net::http::Response, ServerFnError> = {
                    Request::post(&format!("{}/update_metadata", upload_base_url))
                        .json(&json!({
                            "video_uid": uid,
                            "delegated_identity": delegated_identity,
                            "meta": VideoMetadata{
                                title: description.clone(),
                                description: description.clone(),
                                tags: hashtags.join(",")
                            },
                            "post_details": SerializablePostDetailsFromFrontend{
                                is_nsfw,
                                hashtags,
                                description,
                                video_uid: uid.clone(),
                                creator_consent_for_inclusion_in_hot_or_not: enable_hot_or_not,
                            }
                        }))
                        .unwrap()
                        .send()
                        .await
                        .map_err(|e| ServerFnError::new(format!("Failed to send request: {:?}", e)))
                };

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

                VideoUploadSuccessful.send_event(
                    uid,
                    hashtags_len,
                    is_nsfw,
                    enable_hot_or_not,
                    0,
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
                    {move || {
                        let cans_wire = cans_res.get()?.ok()?;
                        let canisters = Canisters::from_wire(cans_wire, expect_context()).ok()?;
                        publish_action.dispatch(canisters);
                        Some(())
                    }}

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
    }.into_any()
}
