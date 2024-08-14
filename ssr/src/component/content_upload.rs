use super::spinner::Spinner;
use crate::{
    auth::DelegatedIdentityWire,
    page::menu::AuthorizedUserToSeedContent,
    state::{canisters::Canisters, content_seed_client::ContentSeedClient},
};
use leptos::*;

#[component]
fn YoutubeUploadInner(canisters: Canisters<true>, #[prop(optional)] url: String) -> impl IntoView {
    let url_value = RwSignal::new(url);
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
                Err(e) => e.to_string(),
                _ => "Submitted!".to_string(),
            };
        }
    });
    let submit_res = on_submit.value();

    view! {
        <div data-hk="1-0-0-3" class="flex h-full items-center justify-around p-4">
            <div data-hk="1-0-0-4" class="flex flex-col items-center justify-center">
                <div class="flex h-full flex-col justify-around gap-6">
                    <div class="flex basis-9/12 flex-col items-center justify-center">
                        <h1 data-hk="1-0-0-5" class="text-2xl md:text-3xl text-white">
                            VIDEO IMPORTER
                        </h1>
                    </div>
                    <div class="flex basis-3/12 flex-col justify-around items-center gap-4">
                        <input
                            type="text"
                            value=move || url_value.get()
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                url_value.set(val);
                            }

                            placeholder=" Paste your link here"
                            class="p-1 md:text-xl"
                        />
                        <button
                            type="submit"
                            class="border border-solid px-4 text-xl md:text-2xl w-fit text-white hover:bg-white hover:text-black"
                            on:click=move |_| on_submit.dispatch(())
                        >

                            Submit
                        </button>
                        <p class="text-base md:text-lg text-white">{move || submit_res().unwrap_or_default()}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn YoutubeUpload(canisters: Canisters<true>, #[prop(optional)] url: String) -> impl IntoView {
    let is_authorized_to_seed_content: AuthorizedUserToSeedContent = expect_context();

    let user_principal = canisters.user_principal();
    let check_authorized = create_resource(
        || (),
        move |_| async move {
            let content_seed_client: ContentSeedClient = expect_context();

            content_seed_client
                .check_if_authorized(user_principal)
                .await
                .unwrap_or_default()
        },
    );

    let canisters_s = store_value(canisters);
    let url_s = store_value(url);

    view! {
        <Suspense fallback=Spinner>
        {move || check_authorized().and_then(move |authorized| {
            is_authorized_to_seed_content.0.set(authorized);
            if !authorized {
                return None;
            }

            Some(view! {
                <YoutubeUploadInner canisters=canisters_s.get_value() url=url_s.get_value()/>
            })
        })}
        </Suspense>
    }
}
