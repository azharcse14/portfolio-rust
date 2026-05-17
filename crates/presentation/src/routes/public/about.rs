use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::SectionHeading;
use crate::data::{EXPERIENCE, SKILLS};
use crate::layouts::PublicLayout;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About — Azharul Islam" />
        <PublicLayout>
            // Bio
            <section class="py-12">
                <SectionHeading
                    eyebrow="About"
                    title="A bit about me"
                    subtitle="Developer who enjoys the whole stack — and writing code I'll still understand six months from now."
                />
                <div class="max-w-3xl space-y-5 text-base leading-relaxed text-slate-700 dark:text-slate-300">
                    <p>
                        "I'm Azharul — a self-driven developer who got into programming through curiosity and stayed for the craft. "
                        "My current focus is fullstack Rust (Leptos, Axum) plus the TypeScript ecosystem when the team needs it."
                    </p>
                    <p>
                        "I care about clean boundaries, fast feedback loops, and shipping software that holds up under maintenance. "
                        "When I'm picking up something new I try to build a real, end-to-end version — even if it's small."
                    </p>
                    <p>
                        "Off the keyboard, I read about systems design, watch too many YouTube deep-dives, and slowly chip away at Bengali short-story translations."
                    </p>
                </div>
            </section>

            // Experience timeline
            <section class="py-16">
                <SectionHeading
                    eyebrow="Journey"
                    title="Experience"
                />
                <ol class="relative border-l border-slate-200 dark:border-slate-700">
                    {EXPERIENCE.iter().map(|e| view! {
                        <li class="mb-10 ml-6">
                            <span class="absolute -left-1.5 mt-1.5 h-3 w-3 rounded-full border-2 border-white bg-indigo-500 dark:border-slate-950" />
                            <p class="text-xs font-semibold uppercase tracking-wide text-indigo-600 dark:text-indigo-400">
                                {e.period}
                            </p>
                            <h3 class="mt-1 text-lg font-semibold text-slate-900 dark:text-slate-50">
                                {e.role}
                                <span class="font-normal text-slate-500 dark:text-slate-400">
                                    " · " {e.company}
                                </span>
                            </h3>
                            <p class="mt-2 text-sm text-slate-600 dark:text-slate-400">
                                {e.description}
                            </p>
                        </li>
                    }).collect_view()}
                </ol>
            </section>

            // Skills grid grouped by category
            <section class="py-16">
                <SectionHeading
                    eyebrow="Toolkit"
                    title="Skills"
                />
                {["Language", "Frontend", "Backend", "Data", "DevOps"].iter().map(|cat| {
                    let cat = *cat;
                    let items = SKILLS.iter().filter(|s| s.category == cat).collect::<Vec<_>>();
                    view! {
                        <div class="mb-8">
                            <h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
                                {cat}
                            </h3>
                            <div class="flex flex-wrap gap-2">
                                {items.into_iter().map(|s| view! {
                                    <span class="rounded-lg bg-slate-100 px-3 py-1.5 text-sm font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-200">
                                        {s.name}
                                    </span>
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </section>
        </PublicLayout>
    }
}
