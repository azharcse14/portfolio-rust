use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::routes::public::{home::HomePage, not_found::NotFoundPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="shortcut icon" href="/favicon.svg" />

                // Google Fonts — Quicksand + Playfair Display (as the legacy template uses)
                <link
                    href="https://fonts.googleapis.com/css?family=Quicksand:300,400,500,700"
                    rel="stylesheet"
                />
                <link
                    href="https://fonts.googleapis.com/css?family=Playfair+Display:400,400i,700"
                    rel="stylesheet"
                />

                // Legacy template stylesheets
                <link rel="stylesheet" href="/css/animate.css" />
                <link rel="stylesheet" href="/css/icomoon.css" />
                <link rel="stylesheet" href="/css/bootstrap.css" />
                <link rel="stylesheet" href="/css/flexslider.css" />
                <link rel="stylesheet" href="/css/owl.carousel.min.css" />
                <link rel="stylesheet" href="/css/owl.theme.default.min.css" />
                <link rel="stylesheet" href="/css/style.css" />

                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <HashedStylesheet options id="leptos" />
                <MetaTags />
            </head>
            <body>
                <App />

                // Legacy template JS bundle — load at end of body to mimic original.
                <script src="/js/jquery.min.js"></script>
                <script src="/js/jquery.easing.1.3.js"></script>
                <script src="/js/bootstrap.min.js"></script>
                <script src="/js/jquery.waypoints.min.js"></script>
                <script src="/js/jquery.flexslider-min.js"></script>
                <script src="/js/owl.carousel.min.js"></script>
                <script src="/js/jquery.countTo.js"></script>
                <script src="/js/main.js"></script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Azharul Islam — Portfolio" />
        <Meta name="description" content="Developer portfolio." />

        <Router>
            <Routes fallback=NotFoundPage>
                <Route path=path!("") view=HomePage />
            </Routes>
        </Router>
    }
}
