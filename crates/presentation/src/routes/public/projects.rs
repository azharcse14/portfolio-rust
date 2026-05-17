use leptos::prelude::*;

use crate::layouts::PublicLayout;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <PublicLayout>
            <h1 class="text-3xl font-bold">"Projects"</h1>
            <p class="mt-4 text-slate-600 dark:text-slate-300">
                "Project list will be loaded from the DB in Phase 3."
            </p>
        </PublicLayout>
    }
}
