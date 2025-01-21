use crate::page::icpump::ai::ICPumpAi;
use crate::page::icpump::ICPumpLanding;

use crate::page::pumpdump::PndProfilePage;
// use crate::page::wallet::TestIndex;
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
        use crate::page::google_redirect::GoogleRedirectHandler;
        view! { <Route path view=GoogleRedirectHandler/> }
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
        use crate::page::google_redirect::GoogleRedirector;
        view! { <Route path view=GoogleRedirector/> }
    }
    #[cfg(not(any(feature = "oauth-ssr", feature = "oauth-hydrate")))]
    {
        view! { <Route path view=NotFound/> }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
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

    // Analytics
    let enable_ga4_script = create_rw_signal(false);
    #[cfg(feature = "ga4")]
    {
        enable_ga4_script.set(true);
        provide_context(EventHistory::default());
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/hot-or-not-leptos-ssr.css"/>

        // sets the document title
        <Title text="Yral"/>

        <Link rel="manifest" href="/app.webmanifest"/>

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
        <Router fallback=|| view! { <NotFound/> }.into_view()>
            <main>
                <Routes>
                    // auth redirect routes exist outside main context
                    <GoogleAuthRedirectHandlerRoute/>
                    <GoogleAuthRedirectorRoute/>
                    <Route path="" view=BaseRoute>
                        <Route path="/" view=RootPage/>
                        <Route path="/hot-or-not/:canister_id/:post_id" view=PostView/>
                        <Route path="/post/:canister_id/:post_id" view=SinglePost/>
                        <Route path="/profile/:canister_id/post/:post_id" view=ProfilePost/>
                        <Route path="/pnd/profile" view=PndProfilePage/>
                        <Route path="/upload" view=UploadPostPage/>
                        <Route path="/error" view=ServerErrorPage/>
                        <Route path="/menu" view=Menu/>
                        <Route path="/settings" view=Settings/>
                        <Route path="/refer-earn" view=ReferEarn/>
                        <Route path="/profile/:id/:tab" view=ProfileView/>
                        <Route path="/profile/:tab" view=ProfileView/>
                        <Route path="/terms-of-service" view=TermsOfService/>
                        <Route path="/privacy-policy" view=PrivacyPolicy/>
                        <Route path="/wallet/:id" view=Wallet/>
                        <Route path="/wallet" view=Wallet/>
                        <Route path="/leaderboard" view=Leaderboard/>
                        <Route path="/logout" view=Logout/>
                        <Route path="/token/create" view=CreateToken/>
                        <Route path="/token/create/settings" view=CreateTokenSettings/>
                        <Route path="/token/create/faq" view=CreateTokenFAQ/>
                        <Route path="/token/info/:token_root/:key_principal" view=TokenInfo/>
                        <Route path="/token/info/:token_root" view=TokenInfo/>
                        <Route path="/token/transfer/:token_root" view=TokenTransfer/>
                        <Route path="/board" view=ICPumpLanding/>
                        <Route path="/icpump-ai" view=ICPumpAi/>
                    // <Route path="/test" view=TestIndex/>
                    </Route>
                </Routes>

            </main>
            <nav>
                <NavBar/>
            </nav>
        </Router>
    }
}
