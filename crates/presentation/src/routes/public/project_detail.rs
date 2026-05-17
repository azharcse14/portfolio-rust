use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_params_map};

use crate::data::find_project;
use crate::layouts::PublicLayout;

#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <PublicLayout>
            {move || {
                let slug = params.read().get("slug").unwrap_or_default();
                match find_project(&slug) {
                    Some(p) => {
                        let title = format!("{} — Azharul Islam", p.title);
                        let gradient_class = format!(
                            "mb-8 flex h-56 items-center justify-center rounded-3xl bg-gradient-to-br {} text-7xl shadow-lg",
                            p.gradient
                        );
                        view! {
                            <Title text=title />
                            <article class="mx-auto max-w-3xl py-12">
                                <A href="/projects" attr:class="mb-6 inline-block text-sm font-medium text-indigo-600 hover:underline dark:text-indigo-400">
                                    "← Back to projects"
                                </A>
                                <div class=gradient_class>
                                    <span>{p.emoji}</span>
                                </div>
                                <div class="mb-3 flex items-center gap-3">
                                    <h1 class="text-4xl font-bold tracking-tight text-slate-900 dark:text-slate-50">
                                        {p.title}
                                    </h1>
                                    {p.featured.then(|| view! {
                                        <span class="rounded-full bg-amber-100 px-2.5 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-900/40 dark:text-amber-300">
                                            "★ Featured"
                                        </span>
                                    })}
                                </div>
                                <p class="mb-6 text-lg text-slate-600 dark:text-slate-400">
                                    {p.tagline}
                                </p>
                                <div class="mb-8 flex flex-wrap gap-2">
                                    {p.tech.iter().map(|t| view! {
                                        <span class="rounded-md bg-slate-100 px-3 py-1 text-sm font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                                            {*t}
                                        </span>
                                    }).collect_view()}
                                </div>
                                <div class="prose prose-slate max-w-none dark:prose-invert">
                                    <p class="text-base leading-relaxed text-slate-700 dark:text-slate-300">
                                        {p.description}
                                    </p>
                                </div>
                                <div class="mt-10 flex gap-3">
                                    {p.github.map(|url| view! {
                                        <a href=url target="_blank" rel="noreferrer"
                                           class="rounded-md bg-slate-900 px-4 py-2 text-sm font-semibold text-white hover:bg-slate-700 dark:bg-white dark:text-slate-900 dark:hover:bg-slate-200">
                                            "View on GitHub →"
                                        </a>
                                    })}
                                    {p.live.map(|url| view! {
                                        <a href=url target="_blank" rel="noreferrer"
                                           class="rounded-md border border-slate-300 px-4 py-2 text-sm font-semibold text-slate-700 hover:bg-slate-50 dark:border-slate-700 dark:text-slate-200 dark:hover:bg-slate-800">
                                            "Live demo ↗"
                                        </a>
                                    })}
                                </div>
                            </article>
                        }.into_any()
                    },
                    None => {
                        #[cfg(feature = "ssr")]
                        {
                            if let Some(opts) = use_context::<leptos_axum::ResponseOptions>() {
                                opts.set_status(axum::http::StatusCode::NOT_FOUND);
                            }
                        }
                        view! {
                            <div class="py-20 text-center">
                                <h1 class="text-3xl font-bold">"Project not found"</h1>
                                <p class="mt-4 text-slate-600 dark:text-slate-400">
                                    "We couldn't find a project with that slug."
                                </p>
                                <A href="/projects" attr:class="mt-6 inline-block text-indigo-600 hover:underline dark:text-indigo-400">
                                    "← Back to projects"
                                </A>
                            </div>
                        }.into_any()
                    },
                }
            }}
        </PublicLayout>
    }
}
