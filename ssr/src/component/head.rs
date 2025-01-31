use crate::component::app_config::AppState;
use leptos::*;

#[component]
pub fn Head() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState not found");

    view! {
        <>
            <link rel="manifest" href=app_state.manifest_path/>
            <link
                rel="icon"
                type="image/png"
                sizes="192x192"
                href=format!("/img/{}/android-chrome-192x192.png", app_state.id)
            />
            <link
                rel="icon"
                type="image/png"
                sizes="384x384"
                href=format!("/img/{}/android-chrome-384x384.png", app_state.id)
            />
            <meta name="theme-color" content=app_state.theme_color/>
            <meta name="application-name" content=app_state.name/>
            <meta name="description" content=app_state.description/>
        </>
    }
}
