use crate::{
    auth::{extract_or_generate_identity, DelegatedIdentityWire},
    state::auth::auth_state,
    try_or_redirect,
};
use leptos::*;

#[component]
pub fn AuthFrame(auth: RwSignal<Option<DelegatedIdentityWire>>) -> impl IntoView {
    let auth_res = create_local_resource(
        || (),
        move |_| async move {
            let identity = try_or_redirect!(extract_or_generate_identity().await);
            auth.set(Some(identity));
        },
    );

    view! { <Suspense>{move || auth_res.get().map(|_| ())}</Suspense> }
}

#[component]
pub fn AuthProvider() -> impl IntoView {
    let auth = auth_state().identity;
    view! {
        <div class="hidden">
            <Show when=move || auth.with(|a| a.is_none())>
                <AuthFrame auth/>
            </Show>
        </div>
    }
}
