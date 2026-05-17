use leptos::prelude::*;

use crate::layouts::PublicLayout;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <PublicLayout>
            <h1 class="text-3xl font-bold">"Blog"</h1>
            <p class="mt-4 text-slate-600 dark:text-slate-300">
                "Blog posts will appear here once Phase 3 lands."
            </p>
        </PublicLayout>
    }
}
