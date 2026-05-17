use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::routes::public::{
    about::AboutPage,
    blog::BlogPage,
    blog_detail::BlogDetailPage,
    contact::ContactPage,
    home::HomePage,
    not_found::NotFoundPage,
    project_detail::ProjectDetailPage,
    projects::ProjectsPage,
};

/// Inline script run before any rendering. Applies stored theme preference
/// (or system default) so there is no flash of unstyled content on first paint.
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
        <html lang="en" class="scroll-smooth">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
                <script>{THEME_BOOT_SCRIPT}</script>
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
            <Routes fallback=NotFoundPage>
                <Route path=path!("") view=HomePage />
                <Route path=path!("about") view=AboutPage />
                <Route path=path!("projects") view=ProjectsPage />
                <Route path=path!("projects/:slug") view=ProjectDetailPage />
                <Route path=path!("blog") view=BlogPage />
                <Route path=path!("blog/:slug") view=BlogDetailPage />
                <Route path=path!("contact") view=ContactPage />
            </Routes>
        </Router>
    }
}
