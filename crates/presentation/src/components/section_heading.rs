use leptos::prelude::*;

#[component]
pub fn SectionHeading(
    title: &'static str,
    #[prop(optional)] subtitle: Option<&'static str>,
    #[prop(optional)] eyebrow: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="mb-10">
            {eyebrow.map(|e| view! {
                <p class="mb-2 text-sm font-semibold uppercase tracking-wider text-indigo-600 dark:text-indigo-400">
                    {e}
                </p>
            })}
            <h2 class="text-3xl font-bold tracking-tight text-slate-900 dark:text-slate-50 sm:text-4xl">
                {title}
            </h2>
            {subtitle.map(|s| view! {
                <p class="mt-3 max-w-2xl text-base text-slate-600 dark:text-slate-400">
                    {s}
                </p>
            })}
        </div>
    }
}
