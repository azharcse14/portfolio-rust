use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::SectionHeading;
use crate::layouts::PublicLayout;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Contact — Azharul Islam" />
        <PublicLayout>
            <section class="mx-auto max-w-2xl py-12">
                <SectionHeading
                    eyebrow="Reach out"
                    title="Get in touch"
                    subtitle="Open to freelance projects, collaboration, or just a chat about Rust + the web."
                />

                <form class="space-y-5 rounded-2xl border border-slate-200 bg-white p-8 shadow-sm dark:border-slate-800 dark:bg-slate-900">
                    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
                        <div>
                            <label for="name" class="mb-1.5 block text-sm font-medium text-slate-700 dark:text-slate-300">
                                "Name"
                            </label>
                            <input
                                type="text"
                                id="name"
                                name="name"
                                required
                                class="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                            />
                        </div>
                        <div>
                            <label for="email" class="mb-1.5 block text-sm font-medium text-slate-700 dark:text-slate-300">
                                "Email"
                            </label>
                            <input
                                type="email"
                                id="email"
                                name="email"
                                required
                                class="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                            />
                        </div>
                    </div>
                    <div>
                        <label for="subject" class="mb-1.5 block text-sm font-medium text-slate-700 dark:text-slate-300">
                            "Subject"
                        </label>
                        <input
                            type="text"
                            id="subject"
                            name="subject"
                            class="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                        />
                    </div>
                    <div>
                        <label for="message" class="mb-1.5 block text-sm font-medium text-slate-700 dark:text-slate-300">
                            "Message"
                        </label>
                        <textarea
                            id="message"
                            name="message"
                            rows="6"
                            required
                            class="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                        />
                    </div>
                    <div class="flex items-center justify-between">
                        <p class="text-xs text-slate-500 dark:text-slate-400">
                            "Form wiring lands in Phase 3 (server fn + Resend)."
                        </p>
                        <button
                            type="submit"
                            disabled
                            class="rounded-md bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white shadow-sm transition hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-white dark:text-slate-900 dark:hover:bg-slate-200"
                        >
                            "Send Message"
                        </button>
                    </div>
                </form>

                <div class="mt-8 text-center text-sm text-slate-500 dark:text-slate-400">
                    "Or email directly at "
                    <a href="mailto:mdazharcse14@gmail.com" class="font-medium text-indigo-600 hover:underline dark:text-indigo-400">
                        "mdazharcse14@gmail.com"
                    </a>
                </div>
            </section>
        </PublicLayout>
    }
}
