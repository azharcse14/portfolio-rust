use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::routes::public::{about::AboutPage, blog::BlogPage, contact::ContactPage, home::HomePage, projects::ProjectsPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="scroll-smooth">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <HashedStylesheet options id="leptos" />
                <MetaTags />
            </head>
            <body class="bg-white text-slate-900 antialiased dark:bg-slate-950 dark:text-slate-100">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Azharul Islam — Portfolio" />
        <Meta name="description" content="Developer portfolio built with Rust + Leptos." />

        <Router>
            <Routes fallback=|| view! { <p class="p-8 text-center">"Page not found."</p> }>
                <Route path=StaticSegment("") view=HomePage />
                <Route path=StaticSegment("about") view=AboutPage />
                <Route path=StaticSegment("projects") view=ProjectsPage />
                <Route path=StaticSegment("blog") view=BlogPage />
                <Route path=StaticSegment("contact") view=ContactPage />
            </Routes>
        </Router>
    }
}
