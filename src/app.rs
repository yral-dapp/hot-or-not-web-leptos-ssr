use crate::{
    component::{base_route::BaseRoute, logout::Logout, nav::NavBar},
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
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(Canisters::default());
    provide_context(PostViewCtx::default());
    let auth_state = AuthState::default();
    provide_context(auth_state.clone());

    // History Tracking
    let history_ctx = HistoryCtx::default();
    provide_context(history_ctx.clone());
    create_effect(move |_| {
        let loc = use_location();
        history_ctx.push(&loc.pathname.get());
        // leptos::logging::log!("{}", history_ctx.log_history());
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/hot-or-not-leptos-ssr.css"/>

        // sets the document title
        <Title text="Yral"/>

        <Link rel="manifest" href="/app.webmanifest"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    // auth redirect routes exist outside main context

                    {#[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
                    {
                        use crate::page::google_redirect::GoogleRedirect;
                        view! { <Route path="/auth/google_redirect" view=GoogleRedirect/> }
                    }}
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
