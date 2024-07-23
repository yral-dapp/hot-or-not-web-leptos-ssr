use crate::{
    auth::DelegatedIdentityWire,
    state::{canisters::Canisters, content_seed_client::ContentSeedClient},
};
use leptos::*;

#[component]
pub fn YoutubeUpload(
    canisters: Canisters<true>,
    #[prop(optional)] url: Option<String>,
) -> impl IntoView {
    let response = RwSignal::new(String::new());
    let url_value = RwSignal::new(String::new());

    if let Some(val) = url {
        url_value.set_untracked(val);
    }

    let create_short_lived_delegated_identity = |canisters: &Canisters<true>| {
        let id = canisters.identity();
        DelegatedIdentityWire::delegate_short_lived_identity(id)
    };

    let on_submit = create_action(move |_| {
        let canisters_copy = canisters.clone();
        async move {
            let delegated_identity = create_short_lived_delegated_identity(&canisters_copy);
            let content_seed_client: ContentSeedClient = expect_context();
            let res = content_seed_client
                .upload_content(url_value(), delegated_identity)
                .await;
            match res {
                Err(e) => response.set(e.to_string()),
                _ => response.set("Submitted!".to_string()),
            };
        }
    });

    view! {
        {move || {
            view! {
                <div data-hk="1-0-0-3" class="flex h-full items-center justify-around p-8">
                    <div
                        data-hk="1-0-0-4"
                        class="flex flex-col items-center justify-center bg-red-700 p-10"
                    >
                        <div class="flex h-full flex-col justify-around">
                            <div class="flex basis-1/2 flex-col mb-10">
                                <h1 data-hk="1-0-0-5" class="text-2xl md:text-3xl text-white">
                                    YOUTUBE UPLOADER
                                </h1>
                            </div>
                            <div class="flex basis-1/2 flex-col justify-around items-center">
                                <input
                                    type="text"
                                    value=move || url_value.get()
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        url_value.set(val);
                                    }

                                    placeholder=" Paste your link here"
                                    class="mb-5 p-1 md:text-xl"
                                />
                                <button
                                    type="submit"
                                    class="my-2 border border-solid px-4 text-xl md:text-2xl w-fit text-white hover:bg-white hover:text-black"
                                    on:click=move |_| on_submit.dispatch(())
                                >

                                    Submit
                                </button>
                                <p class="text-base md: text-lg text-white">{response()}</p>

                            </div>
                        </div>
                    </div>
                </div>
            }
        }}
    }
}
