use crate::{
    component::{base_route::BaseRoute, nav::NavBar},
    error_template::{AppError, ErrorTemplate},
    page::{
        about_us::AboutUs,
        airdrop::Airdrop,
        err::ServerErrorPage,
        faq::Faq,
        menu::Menu,
        post_view::{PostView, PostViewCtx},
        privacy::PrivacyPolicy,
        profile::ProfileView,
        refer_earn::ReferEarn,
        root::RootPage,
        terms::TermsOfService,
        upload::UploadPostPage,
        wallet::*,
    },
    state::{
        auth::{AuthClient, AuthState},
        canisters::Canisters,
    },
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
    provide_context(AuthClient::default());

    view! {
        <Stylesheet id="leptos" href="/pkg/hot-or-not-leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Link rel="manifest" href="/app.webmanifest"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=BaseRoute>
                        <Route path="/hot-or-not/:canister_id/:post_id" view=PostView/>
                        <Route path="/profile/:id" view=ProfileView/>
                        <Route path="/upload" view=UploadPostPage/>
                        <Route path="/error" view=ServerErrorPage/>
                        <Route path="/menu" view=Menu/>
                        <Route path="/airdrop" view=Airdrop/>
                        <Route path="/refer-earn" view=ReferEarn/>
                        <Route path="/about-us" view=AboutUs/>
                        <Route path="/faq" view=Faq/>
                        <Route path="/terms-of-service" view=TermsOfService/>
                        <Route path="/privacy-policy" view=PrivacyPolicy/>
                        <Route path="/wallet" view=Wallet/>
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
