use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-30 border-b border-slate-200 bg-white/80 backdrop-blur dark:border-slate-800 dark:bg-slate-950/80">
            <div class="mx-auto flex max-w-6xl items-center justify-between px-4 py-3">
                <A href="/" attr:class="text-lg font-semibold tracking-tight">
                    "Azharul"
                </A>
                <ul class="flex items-center gap-6 text-sm font-medium text-slate-600 dark:text-slate-300">
                    <li><A href="/about">"About"</A></li>
                    <li><A href="/projects">"Projects"</A></li>
                    <li><A href="/blog">"Blog"</A></li>
                    <li><A href="/contact">"Contact"</A></li>
                </ul>
            </div>
        </nav>
    }
}
