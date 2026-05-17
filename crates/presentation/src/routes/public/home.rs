use leptos::prelude::*;

use crate::layouts::PublicLayout;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <PublicLayout>
            <section class="py-16 text-center">
                <h1 class="text-4xl font-bold tracking-tight sm:text-6xl">
                    "Hello, I'm Azharul."
                </h1>
                <p class="mx-auto mt-6 max-w-2xl text-lg text-slate-600 dark:text-slate-300">
                    "Developer. This portfolio is built with Rust + Leptos following clean architecture."
                </p>
                <div class="mt-10 flex justify-center gap-4">
                    <a
                        href="/projects"
                        class="rounded-md bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white hover:bg-slate-700 dark:bg-white dark:text-slate-900 dark:hover:bg-slate-200"
                    >
                        "View Projects"
                    </a>
                    <a
                        href="/contact"
                        class="rounded-md border border-slate-300 px-5 py-2.5 text-sm font-semibold text-slate-700 hover:bg-slate-50 dark:border-slate-700 dark:text-slate-200 dark:hover:bg-slate-800"
                    >
                        "Contact"
                    </a>
                </div>
            </section>
        </PublicLayout>
    }
}
