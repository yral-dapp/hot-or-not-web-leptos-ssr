use crate::{
    component::{base_route::BaseRoute, logout::Logout, nav::NavBar},
    consts,
    error_template::{AppError, ErrorTemplate},
    page::{
        err::ServerErrorPage,
        leaderboard::Leaderboard,
        menu::Menu,
        post_view::{PostView, PostViewCtx},
        privacy::PrivacyPolicy,
        profile::ProfileView,
        refer_earn::ReferEarn,
        root::RootPage,
        terms::TermsOfService,
        upload::UploadPostPage,
        wallet::{transactions::Transactions, Wallet},
    },
    state::{auth::AuthState, canisters::Canisters, history::HistoryCtx},
    utils::event_streaming::EventHistory,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use yral_auth_client::AuthClient;

#[component]
fn NotFound() -> impl IntoView {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);
    view! { <ErrorTemplate outside_errors/> }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(Canisters::default());
    provide_context(PostViewCtx::default());
    let auth_state = AuthState::default();
    provide_context(auth_state.clone());
    provide_context(AuthClient::with_base_url(consts::AUTH_API_BASE.clone()));

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

        // content for this welcome page
        <Router fallback=|| view! { <NotFound/> }.into_view()>
            <main>
                <Routes>
                    <Route path="/" view=BaseRoute>
                        <Route path="/hot-or-not/:canister_id/:post_id" view=PostView/>
                        <Route path="/profile/:id" view=ProfileView/>
                        <Route path="/upload" view=UploadPostPage/>
                        <Route path="/error" view=ServerErrorPage/>
                        <Route path="/menu" view=Menu/>
                        <Route path="/refer-earn" view=ReferEarn/>
                        <Route path="/terms-of-service" view=TermsOfService/>
                        <Route path="/privacy-policy" view=PrivacyPolicy/>
                        <Route path="/wallet" view=Wallet/>
                        <Route path="/transactions" view=Transactions/>
                        <Route path="/leaderboard" view=Leaderboard/>
                        <Route path="/logout" view=Logout/>
                        <Route path="" view=RootPage/>
                    </Route>
                </Routes>

            </main>
            <nav>
                <NavBar/>
            </nav>
        </Router>
    }
}
