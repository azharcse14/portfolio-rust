use leptos::prelude::*;

use crate::layouts::PublicLayout;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <PublicLayout>
            <h1 class="text-3xl font-bold">"Contact"</h1>
            <p class="mt-4 text-slate-600 dark:text-slate-300">
                "Contact form will be added in Phase 3 — message will save to DB + send via Resend."
            </p>
        </PublicLayout>
    }
}
