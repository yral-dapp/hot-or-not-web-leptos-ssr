use crate::{
    auth::DelegatedIdentityWire,
    component::{
        auth_providers::LoginProvCtx, login_modal::LoginModal, spinner::FullScreenSpinner,
    },
    state::canisters::{authenticated_canisters, Canisters}, utils::MockPartialEq,
};
use ic_agent::{identity::DelegatedIdentity, Identity};
use leptos::*;
use leptos_use::use_window;
use wasm_bindgen::{closure::Closure, JsValue};

use crate::state::auth::account_connected_reader;



#[component]
pub fn YoutubeUpload(#[prop(optional)] url: Option<String>) -> impl IntoView {
    let response = RwSignal::new(String::new());
    let url_value = RwSignal::new(String::new());

    if let Some(val) = url {
        url_value.set(val);
    }
    let can_res = authenticated_canisters();
    let is_authorized = create_local_resource(
        move || {
            MockPartialEq(can_res())
        },
        move |can_res| async move {
            log::warn!("called");
            let canisters = can_res.0?.ok()?;
            let res = canisters.is_user_authorized_to_seed_content().await.ok();
            res
        },
    );

    let create_short_lived_delegated_identity = move || {
        can_res.with(|cans_res| {
            let canisters = cans_res.as_ref().expect("canisters to be present").as_ref().expect("canisters should be present");
            let id = canisters.identity();
            let delegated_identity_wire = DelegatedIdentityWire::delegate_short_lived_identity(id);
            delegated_identity_wire
        })
    };

    let on_submit = create_action( move |_| async move {
        let canisters = can_res().transpose().ok().flatten().expect("canisters to be present");
        let delegated_identity = create_short_lived_delegated_identity();       
        let res = canisters.upload_using_content_seed(delegated_identity, url_value()).await;
        match res {
            Err(e) => {response.set(e.to_string())}
            _ => {
                response.set("Submitted!".to_string())
            }
        };
    });

    view! {
        <Suspense fallback=|| {
            view! { <FullScreenSpinner/> }
        }>

            {move || {
                can_res();
                if is_authorized().flatten() == Some(true) {
                    view! {
                        <div data-hk="1-0-0-3" class="flex h-full items-center justify-around p-8">
                            <div
                                data-hk="1-0-0-4"
                                class="flex flex-col items-center justify-center bg-red-700 p-10"
                            >
                                <div class="flex h-full flex-col justify-around">
                                    <div class="flex basis-1/2 flex-col mb-10">
                                        <h1
                                            data-hk="1-0-0-5"
                                            class="text-2xl md:text-3xl text-white"
                                        >
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
                } else {
                    view! {
                        <div data-hk="1-0-0-3" class="flex h-full items-center justify-around p-8">
                            <h1 class="text-lg md:text-xl text-white">
                                You are not authorized to access this service at the moment
                            </h1>
                        </div>
                    }
                }
            }}

        </Suspense>
    }
}
