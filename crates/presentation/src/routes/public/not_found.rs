use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::layouts::PublicLayout;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    // Return a 404 status on SSR so crawlers and CDNs don't index dead URLs.
    #[cfg(feature = "ssr")]
    {
        if let Some(response_options) = use_context::<leptos_axum::ResponseOptions>() {
            response_options.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <Title text="Page not found — Azharul Islam" />
        <PublicLayout>
            <section class="flex flex-col items-center justify-center py-32 text-center">
                <p class="text-7xl font-bold text-slate-300 dark:text-slate-700">"404"</p>
                <h1 class="mt-4 text-3xl font-bold tracking-tight text-slate-900 dark:text-slate-50">
                    "Page not found"
                </h1>
                <p class="mt-3 max-w-md text-slate-600 dark:text-slate-400">
                    "The page you're looking for doesn't exist, was moved, or never existed in the first place."
                </p>
                <A
                    href="/"
                    attr:class="mt-8 rounded-md bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-slate-700 dark:bg-white dark:text-slate-900 dark:hover:bg-slate-200"
                >
                    "← Back home"
                </A>
            </section>
        </PublicLayout>
    }
}
