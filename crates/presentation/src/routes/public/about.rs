use leptos::prelude::*;

use crate::layouts::PublicLayout;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <PublicLayout>
            <h1 class="text-3xl font-bold">"About"</h1>
            <p class="mt-4 text-slate-600 dark:text-slate-300">
                "About content will be wired to the database in Phase 3."
            </p>
        </PublicLayout>
    }
}
