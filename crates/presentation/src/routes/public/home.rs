use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::components::{ProjectCard, SectionHeading};
use crate::data::{PROJECTS, SKILLS};
use crate::layouts::PublicLayout;

#[component]
pub fn HomePage() -> impl IntoView {
    let featured = PROJECTS.iter().filter(|p| p.featured).collect::<Vec<_>>();

    view! {
        <Title text="Azharul Islam — Developer" />
        <PublicLayout>
            // Hero
            <section class="relative isolate overflow-hidden py-20 sm:py-28">
                <div class="absolute inset-x-0 -top-10 -z-10 transform-gpu blur-3xl">
                    <div class="mx-auto aspect-[1155/678] w-[40rem] -translate-x-1/2 rotate-30 bg-gradient-to-tr from-indigo-400 to-pink-400 opacity-20 dark:opacity-10"
                         style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%);" />
                </div>

                <div class="text-center">
                    <p class="inline-flex items-center gap-2 rounded-full border border-slate-200 bg-white/60 px-3 py-1 text-xs font-medium text-slate-600 backdrop-blur dark:border-slate-700 dark:bg-slate-900/60 dark:text-slate-300">
                        <span class="h-2 w-2 rounded-full bg-emerald-500" />
                        "Available for freelance work"
                    </p>
                    <h1 class="mt-6 text-5xl font-bold tracking-tight text-slate-900 dark:text-slate-50 sm:text-7xl">
                        "Hi, I'm "
                        <span class="bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 bg-clip-text text-transparent">
                            "Azharul"
                        </span>
                        "."
                    </h1>
                    <p class="mx-auto mt-6 max-w-2xl text-lg leading-relaxed text-slate-600 dark:text-slate-300">
                        "Full-stack developer who likes building fast, well-architected things. "
                        "This site is built with Rust + Leptos and follows clean architecture."
                    </p>
                    <div class="mt-10 flex flex-wrap justify-center gap-3">
                        <A
                            href="/projects"
                            attr:class="rounded-md bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-slate-700 dark:bg-white dark:text-slate-900 dark:hover:bg-slate-200"
                        >
                            "View Projects →"
                        </A>
                        <A
                            href="/contact"
                            attr:class="rounded-md border border-slate-300 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 shadow-sm transition hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
                        >
                            "Get in Touch"
                        </A>
                    </div>
                </div>
            </section>

            // Featured projects
            <section class="py-16">
                <SectionHeading
                    eyebrow="Work"
                    title="Featured projects"
                    subtitle="A few things I've built recently. More in the projects archive."
                />
                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    {featured.into_iter().map(|p| view! {
                        <ProjectCard project=p />
                    }).collect_view()}
                </div>
                <div class="mt-10 text-center">
                    <A
                        href="/projects"
                        attr:class="text-sm font-semibold text-indigo-600 hover:underline dark:text-indigo-400"
                    >
                        "See all projects →"
                    </A>
                </div>
            </section>

            // Skills strip
            <section class="py-16">
                <SectionHeading
                    eyebrow="Stack"
                    title="Things I work with"
                    subtitle="Tools I reach for most often, across the stack."
                />
                <div class="flex flex-wrap justify-center gap-2">
                    {SKILLS.iter().map(|s| view! {
                        <span class="rounded-full border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200">
                            {s.name}
                        </span>
                    }).collect_view()}
                </div>
            </section>

            // CTA
            <section class="my-16 rounded-3xl bg-gradient-to-br from-indigo-600 to-purple-700 p-12 text-center text-white shadow-xl">
                <h2 class="text-3xl font-bold sm:text-4xl">"Have a project in mind?"</h2>
                <p class="mx-auto mt-4 max-w-xl text-indigo-100">
                    "I'm open to interesting fullstack work — especially in Rust, TypeScript, or anything where craft matters."
                </p>
                <A
                    href="/contact"
                    attr:class="mt-8 inline-block rounded-md bg-white px-6 py-3 text-sm font-semibold text-indigo-700 shadow-sm transition hover:bg-indigo-50"
                >
                    "Start a conversation →"
                </A>
            </section>
        </PublicLayout>
    }
}
