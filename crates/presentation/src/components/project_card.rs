use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::SampleProject;

#[component]
pub fn ProjectCard(project: &'static SampleProject) -> impl IntoView {
    let href = format!("/projects/{}", project.slug);
    let gradient_class = format!(
        "flex h-32 items-center justify-center rounded-t-2xl bg-gradient-to-br {} text-5xl",
        project.gradient
    );

    view! {
        <A
            href=href
            attr:class="group flex flex-col overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm transition hover:shadow-lg dark:border-slate-800 dark:bg-slate-900"
        >
            <div class=gradient_class>
                <span>{project.emoji}</span>
            </div>
            <div class="flex flex-1 flex-col gap-3 p-5">
                <div class="flex items-start justify-between gap-3">
                    <h3 class="text-lg font-semibold text-slate-900 dark:text-slate-50 group-hover:underline">
                        {project.title}
                    </h3>
                    {project.featured.then(|| view! {
                        <span class="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-900/40 dark:text-amber-300">
                            "★ Featured"
                        </span>
                    })}
                </div>
                <p class="text-sm text-slate-600 dark:text-slate-400">
                    {project.tagline}
                </p>
                <div class="mt-auto flex flex-wrap gap-1.5 pt-2">
                    {project.tech.iter().take(4).map(|t| view! {
                        <span class="rounded-md bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                            {*t}
                        </span>
                    }).collect_view()}
                </div>
            </div>
        </A>
    }
}
