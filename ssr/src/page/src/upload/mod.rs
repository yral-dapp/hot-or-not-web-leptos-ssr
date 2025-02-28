mod cf_upload;
mod validators;
mod video_upload;
use leptos_meta::*;

use component::toggle::ToggleWithLabel;

use utils::{
    event_streaming::events::auth_canisters_store,
    event_streaming::events::{VideoUploadInitiated, VideoUploadUploadButtonClicked},
    host::{show_cdao_page, show_pnd_page},
    web::FileWithUrl,
};

use leptos::{
    html::{Input, Textarea},
    prelude::*,
};

use leptos_router::components::Redirect;
use validators::{description_validator, hashtags_validator};
use video_upload::{PreVideoUpload, VideoUploader};

#[derive(Clone)]
struct UploadParams {
    file_blob: FileWithUrl,
    hashtags: Vec<String>,
    description: String,
    enable_hot_or_not: bool,
    is_nsfw: bool,
}

#[component]
fn PreUploadView(trigger_upload: WriteSignal<Option<UploadParams>, LocalStorage>) -> impl IntoView {
    let description_err = RwSignal::new(String::new());
    let desc_err_memo = Memo::new(move |_| description_err());
    let hashtags = RwSignal::new(Vec::new());
    let hashtags_err = RwSignal::new(String::new());
    let hashtags_err_memo = Memo::new(move |_| hashtags_err());
    let file_blob = RwSignal::new_local(None::<FileWithUrl>);
    let desc = NodeRef::<Textarea>::new();
    let invalid_form = Memo::new(move |_| {
        // Description error
        !desc_err_memo.with(|desc_err_memo| desc_err_memo.is_empty())
                // Hashtags error
                || !hashtags_err_memo.with(|hashtags_err_memo| hashtags_err_memo.is_empty())
                // File is not uploaded
                || file_blob.with(|file_blob| file_blob.is_none())
                // Hashtags are empty
                || hashtags.with(|hashtags| hashtags.is_empty())
                // Description is empty
                || desc.get().map(|d| d.value().is_empty()).unwrap_or(true)
    });
    let hashtag_inp = NodeRef::<Input>::new();
    let enable_hot_or_not = NodeRef::<Input>::new();
    let is_nsfw = NodeRef::<Input>::new();
    let canister_store = auth_canisters_store();

    VideoUploadInitiated.send_event();

    let on_submit = move || {
        VideoUploadUploadButtonClicked.send_event(
            hashtag_inp,
            is_nsfw,
            enable_hot_or_not,
            canister_store,
        );

        let description = desc.get_untracked().unwrap().value();
        let hashtags = hashtags.get_untracked();
        let Some(file_blob) = file_blob.get_untracked() else {
            return;
        };
        trigger_upload.set(Some(UploadParams {
            file_blob,
            hashtags,
            description,
            enable_hot_or_not: false,
            is_nsfw: is_nsfw
                .get_untracked()
                .map(|v| v.checked())
                .unwrap_or_default(),
        }));
    };

    let hashtag_on_input = move |hts| match hashtags_validator(hts) {
        Ok(hts) => {
            hashtags.set(hts);
            hashtags_err.set(String::new());
        }
        Err(e) => hashtags_err.set(e),
    };

    Effect::new(move |_| {
        let Some(hashtag_inp) = hashtag_inp.get() else {
            return;
        };

        let val = hashtag_inp.value();
        if !val.is_empty() {
            hashtag_on_input(val);
        }
    });

    view! {
        <PreVideoUpload file_blob=file_blob.write_only() />
        <div class="flex flex-col gap-4 lg:basis-7/12">
            <div class="flex flex-col gap-y-2">
                <Show when=move || { description_err.with(| description_err | ! description_err.is_empty()) }>
                    <span class="text-red-500 text-sm">{desc_err_memo()}</span>
                </Show>
                <textarea
                    node_ref=desc
                    on:input=move |ev| {
                        let desc = event_target_value(&ev);
                        description_err.set(description_validator(desc).err().unwrap_or_default());
                    }

                    class="p-4 bg-neutral-800 rounded-md min-w-full"
                    rows=3
                    placeholder="Write your description here.."
                ></textarea>
            </div>
            <div class="flex flex-col gap-y-2">
                <Show
                    when=move || { hashtags_err.with(| hashtags_err | ! hashtags_err.is_empty()) }
                    fallback=|| {
                        view! { <h3 class="font-semibold text-neutral-600">Add Hashtags</h3> }
                    }
                >

                    <h3 class="text-red-500 font-semibold">{hashtags_err_memo()}</h3>
                </Show>
                <input
                    node_ref=hashtag_inp
                    on:input=move |ev| {
                        let hts = event_target_value(&ev);
                        hashtag_on_input(hts);
                    }

                    class="p-4 py-5 bg-neutral-800 rounded-md"
                    type="text"
                    placeholder="#hashtag1,#hashtag2,#hashtag3..."
                />
            </div>
            <div class="flex flex-col gap-y-2">
                // <ToggleWithLabel node_ref=enable_hot_or_not lab="Participate in Hot or Not"/>
                <ToggleWithLabel lab="NSFW" />
            </div>
            <button
                on:click=move |_| on_submit()
                disabled=invalid_form
                class="py-3 w-5/6 md:w-4/6 my-8 self-center disabled:bg-primary-400 disabled:text-white/80 bg-primary-600 rounded-full font-bold text-md md:text-lg lg:text-xl"
            >
                Upload Video
            </button>
        </div>
    }
}

#[component]
pub fn CreatorDaoCreatePage() -> impl IntoView {
    view! { <Redirect path="/token/create" /> }
}

#[component]
pub fn YralUploadPostPage() -> impl IntoView {
    let trigger_upload = RwSignal::new_local(None::<UploadParams>);

    view! {
        <Title text="YRAL - Upload" />
        <div class="flex flex-col min-h-dvh w-dvw items-center overflow-y-scroll gap-6 md:gap-8 lg:gap-16 pb-12 pt-4 md:pt-6 px-3 md:px-6 lg:px-10 bg-black text-white">
            <div class="w-full flex justify-center items-center relative h-12">
            <h1 class="text-xl font-bold">Upload</h1>
            <img src="/img/logo.webp" class="absolute block sm:hidden top-0 left-0 w-12 h-12" />
            <img src="/img/logo-mark.webp" class="hidden absolute sm:block top-0 left-0 h-12" />
            </div>
            <div class="flex flex-col lg:flex-row place-content-center min-h-full w-full">
                <Show
                    when=move || { trigger_upload.with(| trigger_upload | trigger_upload.is_some()) }
                    fallback=move || {
                        view! { <PreUploadView trigger_upload=trigger_upload.write_only() /> }
                    }
                >

                    <VideoUploader params=trigger_upload.get_untracked().unwrap() />
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn UploadPostPage() -> impl IntoView {
    if show_cdao_page() || show_pnd_page() {
        view! { <CreatorDaoCreatePage /> }.into_any()
    } else {
        view! { <YralUploadPostPage /> }.into_any()
    }
}
