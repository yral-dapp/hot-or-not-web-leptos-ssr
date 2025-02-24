use crate::page::icpump::ai::ICPumpAi;
use crate::page::icpump::ICPumpLanding;
use crate::page::pumpdump::{withdrawal, PndProfilePage};
use crate::state::app_type::AppType;
use crate::utils::host::show_preview_component;
// use crate::page::wallet::TestIndex;
use crate::state::app_state::AppState;
use crate::{
    component::{base_route::BaseRoute, nav::NavBar},
    error_template::{AppError, ErrorTemplate},
    page::{
        err::ServerErrorPage,
        leaderboard::Leaderboard,
        logout::Logout,
        menu::{AuthorizedUserToSeedContent, Menu},
        post_view::{single_post::SinglePost, PostView, PostViewCtx},
        privacy::PrivacyPolicy,
        profile::{profile_post::ProfilePost, ProfilePostsContext, ProfileView},
        refer_earn::ReferEarn,
        root::RootPage,
        settings::Settings,
        terms::TermsOfService,
        token::{
            create::{CreateToken, CreateTokenCtx, CreateTokenSettings},
            create_token_faq::CreateTokenFAQ,
            info::TokenInfo,
            transfer::TokenTransfer,
        },
        upload::UploadPostPage,
        wallet::Wallet,
    },
    state::{audio_state::AudioState, content_seed_client::ContentSeedClient, history::HistoryCtx},
    utils::event_streaming::EventHistory,
};
use leptos::either::Either;
use leptos_router::hooks::use_location;
use leptos_router::{components::*, path, MatchNestedRoutes};
use yral_canisters_common::Canisters;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;

#[component]
fn NotFound() -> impl IntoView {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! { <ErrorTemplate outside_errors/> }
}

#[component(transparent)]
fn GoogleAuthRedirectHandlerRoute() -> impl MatchNestedRoutes + Clone {
    let path = path!("/auth/google_redirect");
    #[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
    {
        // if show_preview_component() {
        //     use crate::page::preview_google_redirect::PreviewGoogleRedirectHandler;
        //     view! { <Route path view=PreviewGoogleRedirectHandler/> }.into_inner()
        // } else {

        // }
        use crate::page::google_redirect::GoogleRedirectHandler;
        view! { <Route path view=GoogleRedirectHandler/> }.into_inner()
    }
    #[cfg(not(any(feature = "oauth-ssr", feature = "oauth-hydrate")))]
    {
        view! { <Route path view=NotFound/> }.into_inner()
    }
}

#[component(transparent)]
fn GoogleAuthRedirectorRoute() -> impl MatchNestedRoutes + Clone {
    let path = path!("/auth/perform_google_redirect");
    #[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
    {
        // if show_preview_component() {
        //     use crate::page::preview_google_redirect::PreviewGoogleRedirector;
        //     view! { <Route path view=PreviewGoogleRedirector/> }.into_inner()
        // } else {

        // }
        use crate::page::google_redirect::GoogleRedirector;
        view! { <Route path view=GoogleRedirector/> }.into_inner()
    }
    #[cfg(not(any(feature = "oauth-ssr", feature = "oauth-hydrate")))]
    {
        view! { <Route path view=NotFound/> }.into_inner()
    }
}


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let app_type = AppType::select();
    let app_state = AppState::from_type(&app_type);
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
    Effect::new(move |_| {
        let loc = use_location();
        history_ctx.push(&loc.pathname.get());
    });

    // Analytics
    let enable_ga4_script = RwSignal::new(false);
    #[cfg(feature = "ga4")]
    {
        enable_ga4_script.set(true);
        provide_context(EventHistory::default());
    }

    view! {
            <Stylesheet id="leptos" href="/pkg/hot-or-not-leptos-ssr.css"/>
            <Title text=app_state.name/>

            // Favicon
            <Link rel="icon" type_="image/svg+xml" href=format!("/{}.svg", app_state.favicon_filename) />
            <Link rel="shortcut icon" href=format!("/{}.ico", app_state.favicon_filename) />
            <Link rel="apple-touch-icon" sizes="180x180" href=format!("/{}-apple.png", app_state.favicon_filename) />

            // Meta
            <Meta name="apple-mobile-web-app-title" content=app_state.name />

            // App manifest
            <Link rel="manifest" href=app_state.manifest_config()/>

            // GA4 Global Site Tag (gtag.js) - Google Analytics
            // G-6W5Q2MRX0E to test locally | G-PLNNETMSLM
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

            // <Script src="https://www.gstatic.com/firebasejs/10.14.1/firebase-app.js"></Script>
            // <Script src="https://www.gstatic.com/firebasejs/10.14.1/firebase-firestore.js"></Script>

            // content for this welcome page
            <body class="bg-black" id="body"/>
            <Router>
            <main>
                <Routes fallback=|| view! { <NotFound/> }.into_view()>
                    // auth redirect routes exist outside main context
                    <GoogleAuthRedirectHandlerRoute/>
                    <GoogleAuthRedirectorRoute/>
                    <ParentRoute path=path!("") view=BaseRoute>
                        <Route path=path!("/") view=RootPage/>
                        <Route path=path!("/hot-or-not/:canister_id/:post_id") view=PostView/>
                        <Route path=path!("/post/:canister_id/:post_id") view=SinglePost/>
                        <Route path=path!("/profile/:canister_id/post/:post_id") view=ProfilePost/>
                        <Route path=path!("/pnd/profile") view=PndProfilePage/>
                        <Route path=path!("/upload") view=UploadPostPage/>
                        <Route path=path!("/error") view=ServerErrorPage/>
                        <Route path=path!("/menu") view=Menu/>
                        <Route path=path!("/settings") view=Settings/>
                        <Route path=path!("/refer-earn") view=ReferEarn/>
                        <Route path=path!("/profile/:id/:tab") view=ProfileView/>
                        <Route path=path!("/profile/:tab") view=ProfileView/>
                        <Route path=path!("/terms-of-service") view=TermsOfService/>
                        <Route path=path!("/privacy-policy") view=PrivacyPolicy/>
                        <Route path=path!("/wallet/:id") view=Wallet/>
                        <Route path=path!("/leaderboard") view=Leaderboard/>
                        <Route path=path!("/logout") view=Logout/>
                        <Route path=path!("/token/create") view=CreateToken/>
                        <Route path=path!("/token/create/settings") view=CreateTokenSettings/>
                        <Route path=path!("/token/create/faq") view=CreateTokenFAQ/>
                        <Route path=path!("/token/info/:token_root/:key_principal") view=TokenInfo/>
                        <Route path=path!("/token/info/:token_root") view=TokenInfo/>
                        <Route path=path!("/token/transfer/:token_root") view=TokenTransfer/>
                        <Route path=path!("/board") view=ICPumpLanding/>
                        <Route path=path!("/icpump-ai") view=ICPumpAi/>
                        <Route path=path!("/pnd/withdraw") view=withdrawal::PndWithdrawal />
                        <Route path=path!("/pnd/withdraw/success") view=withdrawal::result::Success />
                        <Route path=path!("/pnd/withdraw/failure") view=withdrawal::result::Failure />
                        // {
                        //     #[cfg(any(feature = "local-bin", feature = "local-lib"))]
                        //     view! {
                        //         <Route path=path!("/pnd/test/:token_root") view=crate::page::pumpdump::PndTest />
                        //     }
                        // }
                    // <Route path="/test" view=TestIndex/>
                    </ParentRoute>
                </Routes>

            </main>
            <nav>
                <NavBar/>
            </nav>
        </Router>
    }
}
