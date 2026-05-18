use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::routes::{
    admin::{
        AdminDashboardPage, AdminMessagesPage, AdminPostEditPage, AdminPostsPage,
        AdminProfilePage, AdminProjectEditPage, AdminProjectsPage, AdminSkillsPage, LoginPage,
        SetupPage,
    },
    public::{home::HomePage, not_found::NotFoundPage},
};

const THEME_BOOT_SCRIPT: &str = r#"
(function() {
  try {
    var stored = localStorage.getItem('theme');
    var prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    var dark = stored ? stored === 'dark' : prefersDark;
    if (dark) document.documentElement.classList.add('dark');
    function toggle() {
      var root = document.documentElement;
      var isDark = root.classList.toggle('dark');
      try { localStorage.setItem('theme', isDark ? 'dark' : 'light'); } catch(e) {}
    }
    window.__toggleTheme = toggle;
    document.addEventListener('click', function(e) {
      var target = e.target;
      while (target && target !== document) {
        if (target.id === 'theme-toggle') { toggle(); return; }
        target = target.parentNode;
      }
    });
  } catch (e) {}
})();
"#;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="shortcut icon" href="/favicon.svg" />

                <link
                    href="https://fonts.googleapis.com/css?family=Quicksand:300,400,500,700"
                    rel="stylesheet"
                />
                <link
                    href="https://fonts.googleapis.com/css?family=Playfair+Display:400,400i,700"
                    rel="stylesheet"
                />

                // Legacy Jackson template — drives the public "/" page only.
                // Admin pages use #admin-app scoping to keep these styles out.
                <link rel="stylesheet" href="/css/animate.css" />
                <link rel="stylesheet" href="/css/icomoon.css" />
                <link rel="stylesheet" href="/css/bootstrap.css" />
                <link rel="stylesheet" href="/css/flexslider.css" />
                <link rel="stylesheet" href="/css/owl.carousel.min.css" />
                <link rel="stylesheet" href="/css/owl.theme.default.min.css" />
                <link rel="stylesheet" href="/css/style.css" />

                // Admin dashboard styles — only matches inside #admin-app.
                <link rel="stylesheet" href="/admin.css" />

                <script>{THEME_BOOT_SCRIPT}</script>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <HashedStylesheet options id="leptos" />
                <MetaTags />
            </head>
            <body>
                <App />

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

                // Admin
                <Route path=path!("admin") view=AdminDashboardPage />
                <Route path=path!("admin/login") view=LoginPage />
                <Route path=path!("admin/setup") view=SetupPage />
                <Route path=path!("admin/profile") view=AdminProfilePage />
                <Route path=path!("admin/projects") view=AdminProjectsPage />
                <Route path=path!("admin/projects/:slug") view=AdminProjectEditPage />
                <Route path=path!("admin/posts") view=AdminPostsPage />
                <Route path=path!("admin/posts/:slug") view=AdminPostEditPage />
                <Route path=path!("admin/skills") view=AdminSkillsPage />
                <Route path=path!("admin/messages") view=AdminMessagesPage />
            </Routes>
        </Router>
    }
}
