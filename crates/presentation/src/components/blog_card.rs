use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::SamplePost;

#[component]
pub fn BlogCard(post: &'static SamplePost) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);

    view! {
        <A
            href=href
            attr:class="group flex flex-col gap-3 rounded-2xl border border-slate-200 bg-white p-6 transition hover:border-slate-300 hover:shadow-md dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700"
        >
            <div class="flex items-center gap-3 text-xs text-slate-500 dark:text-slate-400">
                <time>{post.date}</time>
                <span>"·"</span>
                <span>{format!("{} min read", post.read_minutes)}</span>
            </div>
            <h3 class="text-xl font-semibold text-slate-900 dark:text-slate-50 group-hover:underline">
                {post.title}
            </h3>
            <p class="text-sm text-slate-600 dark:text-slate-400">
                {post.excerpt}
            </p>
            <div class="flex flex-wrap gap-1.5 pt-2">
                {post.tags.iter().map(|t| view! {
                    <span class="rounded-full bg-slate-100 px-2.5 py-0.5 text-xs font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                        {*t}
                    </span>
                }).collect_view()}
            </div>
        </A>
    }
}
