use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_params_map};
use pulldown_cmark::{html, Options, Parser};

use crate::data::find_post;
use crate::layouts::PublicLayout;

fn render_markdown(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

#[component]
pub fn BlogDetailPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <PublicLayout>
            {move || {
                let slug = params.read().get("slug").unwrap_or_default();
                match find_post(&slug) {
                    Some(p) => {
                        let title = format!("{} — Azharul Islam", p.title);
                        let html_body = render_markdown(p.body_md);
                        view! {
                            <Title text=title />
                            <article class="mx-auto max-w-3xl py-12">
                                <A href="/blog" attr:class="mb-6 inline-block text-sm font-medium text-indigo-600 hover:underline dark:text-indigo-400">
                                    "← Back to blog"
                                </A>
                                <div class="mb-3 flex items-center gap-3 text-sm text-slate-500 dark:text-slate-400">
                                    <time>{p.date}</time>
                                    <span>"·"</span>
                                    <span>{format!("{} min read", p.read_minutes)}</span>
                                </div>
                                <h1 class="mb-4 text-4xl font-bold tracking-tight text-slate-900 dark:text-slate-50">
                                    {p.title}
                                </h1>
                                <div class="mb-8 flex flex-wrap gap-2">
                                    {p.tags.iter().map(|t| view! {
                                        <span class="rounded-full bg-indigo-100 px-3 py-1 text-xs font-medium text-indigo-700 dark:bg-indigo-900/40 dark:text-indigo-300">
                                            {*t}
                                        </span>
                                    }).collect_view()}
                                </div>
                                <div
                                    class="prose prose-slate max-w-none dark:prose-invert prose-headings:text-slate-900 dark:prose-headings:text-slate-50 prose-a:text-indigo-600 dark:prose-a:text-indigo-400 prose-code:rounded prose-code:bg-slate-100 prose-code:px-1 prose-code:py-0.5 prose-code:text-slate-800 dark:prose-code:bg-slate-800 dark:prose-code:text-slate-200 prose-pre:bg-slate-900 prose-pre:text-slate-100"
                                    inner_html=html_body
                                />
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
                                <h1 class="text-3xl font-bold">"Post not found"</h1>
                                <A href="/blog" attr:class="mt-6 inline-block text-indigo-600 hover:underline dark:text-indigo-400">
                                    "← Back to blog"
                                </A>
                            </div>
                        }.into_any()
                    },
                }
            }}
        </PublicLayout>
    }
}
