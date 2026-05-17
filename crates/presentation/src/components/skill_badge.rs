use leptos::prelude::*;

#[component]
pub fn SkillBadge(
    name: &'static str,
    #[prop(optional)] category: Option<&'static str>,
) -> impl IntoView {
    view! {
        <span class="inline-flex items-center gap-1.5 rounded-lg border border-slate-200 bg-white px-3 py-1.5 text-sm font-medium text-slate-700 shadow-sm dark:border-slate-700 dark:bg-slate-800 dark:text-slate-200">
            <span>{name}</span>
            {category.map(|c| view! {
                <span class="text-xs text-slate-400 dark:text-slate-500">{c}</span>
            })}
        </span>
    }
}
