use crate::component::app_config::{AppState, AppType};
use crate::page::icpump::ai::ICPumpAi;
use crate::page::icpump::ICPumpLanding;

use crate::utils::host::{get_host, show_preview_component};
// use crate::page::wallet::TestIndex;
use crate::{
    component::{base_route::BaseRoute, nav::NavBar},
    error_template::{AppError, ErrorTemplate},
    page::{
        leaderboard::Leaderboard,
        menu::{AuthorizedUserToSeedContent, Menu},
        post_view::{single_post::SinglePost, PostView, PostViewCtx},
        profile::{ProfilePostsContext, ProfileView},
        root::RootPage,
        settings::Settings,
        token::{
            create::{CreateToken, CreateTokenCtx},
            info::TokenInfo,
            transfer::TokenTransfer,
        },
        upload::UploadPostPage,
        wallet::Wallet,
    },
    state::{audio_state::AudioState, content_seed_client::ContentSeedClient, history::HistoryCtx},
    utils::event_streaming::EventHistory,
};
use yral_canisters_common::Canisters;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn NotFound() -> impl IntoView {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! { <ErrorTemplate outside_errors/> }
}

#[component(transparent)]
fn GoogleAuthRedirectHandlerRoute() -> impl IntoView {
    let path = "/auth/google_redirect";
    #[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
    {
        if show_preview_component() {
            use crate::page::google_redirect::PreviewGoogleRedirectHandler;
            view! { <Route path view=PreviewGoogleRedirectHandler/> }
        } else {
            use crate::page::google_redirect::GoogleRedirectHandler;
            view! { <Route path view=GoogleRedirectHandler/> }
        }
    }
    #[cfg(not(any(feature = "oauth-ssr", feature = "oauth-hydrate")))]
    {
        view! { <Route path view=NotFound/> }
    }
}

#[component(transparent)]
fn GoogleAuthRedirectorRoute() -> impl IntoView {
    let path = "/auth/perform_google_redirect";
    #[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
    {
        if show_preview_component() {
            use crate::page::google_redirect::PreviewGoogleRedirector;
            view! { <Route path view=PreviewGoogleRedirector/> }
        } else {
            use crate::page::google_redirect::GoogleRedirector;
            view! { <Route path view=GoogleRedirector/> }
        }
    }
    #[cfg(not(any(feature = "oauth-ssr", feature = "oauth-hydrate")))]
    {
        view! { <Route path view=NotFound/> }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let host = get_host();
    let app_state = AppState::from_host(&host);
    provide_context(app_state.clone());

    // Existing context providers
    provide_context(Canisters::default());
    provide_context(ContentSeedClient::default());
    provide_context(PostViewCtx::default());
    provide_context(ProfilePostsContext::default());
    provide_context(AuthorizedUserToSeedContent::default());
    provide_context(AudioState::default());
    provide_context(CreateTokenCtx::default());

    #[cfg(feature = "hydrate")]
    {
        use crate::utils::ml_feed::ml_feed_grpcweb::MLFeed;
        provide_context(MLFeed::default());
    }

    // History Tracking
    let history_ctx = HistoryCtx::default();
    provide_context(history_ctx.clone());
    create_effect(move |_| {
        let loc = use_location();
        history_ctx.push(&loc.pathname.get());
    });

    // Analytics setup
    let enable_ga4_script = create_rw_signal(false);
    #[cfg(feature = "ga4")]
    {
        enable_ga4_script.set(true);
        provide_context(EventHistory::default());
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/hot-or-not-leptos-ssr.css"/>
        <Title text=app_state.name/>
        <Link rel="manifest" href=app_state.manifest_path/>
        <meta name="theme-color" content=app_state.theme_color/>

        // GA4 setup
        <Show when=enable_ga4_script>
            <Script
                async_="true"
                src=concat!("https://www.googletagmanager.com/gtag/js?id=", "G-PLNNETMSLM")
            />
            <Script>
                {r#"
                window.dataLayer = window.dataLayer || [];
                function gtag(){dataLayer.push(arguments);}
                gtag('js', new Date());
                gtag('config', 'G-PLNNETMSLM');
                "#}
            </Script>
        </Show>

        <Router fallback=|| view! { <NotFound/> }.into_view()>
            <main>
                <Routes>
                    <GoogleAuthRedirectHandlerRoute/>
                    <GoogleAuthRedirectorRoute/>

                    {move || match app_state.app_type {
                        AppType::HotOrNot => view! {
                            <Route path="" view=BaseRoute>
                                <Route path="/hot-or-not/:canister_id/:post_id" view=PostView/>
                                <Route path="/post/:canister_id/:post_id" view=SinglePost/>
                            </Route>
                        },
                        AppType::ICPump => view! {
                            <Route path="" view=BaseRoute>
                                <Route path="/board" view=ICPumpLanding/>
                                <Route path="/icpump-ai" view=ICPumpAi/>
                                <Route path="/token/create" view=CreateToken/>
                                <Route path="/token/info/:token_root/:key_principal" view=TokenInfo/>
                                <Route path="/token/transfer/:token_root" view=TokenTransfer/>
                            </Route>
                        },
                        AppType::YRAL => view! {
                            <Route path="" view=BaseRoute>
                                <Route path="/" view=RootPage/>
                                <Route path="/upload" view=UploadPostPage/>
                                <Route path="/profile/:id/:tab" view=ProfileView/>
                                <Route path="/wallet/:id" view=Wallet/>
                                <Route path="/menu" view=Menu/>
                                <Route path="/settings" view=Settings/>
                                <Route path="/leaderboard" view=Leaderboard/>
                            </Route>
                        }
                    }}
                </Routes>
            </main>
            <nav>
                <NavBar/>
            </nav>
        </Router>
    }
}
