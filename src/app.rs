use crate::{
    error_template::{AppError, ErrorTemplate},
    route::root::RootPage,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Stylesheet id="leptos" href="/pkg/hot-or-not-web-leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Hot or Not"/>

        <Meta
            name="description"
            content="Hot or Not's upcoming blazing fast web app that can do it all."
        />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=RootPage/>
                </Routes>
            </main>
        </Router>
    }
}

