use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-24 border-t border-slate-200 dark:border-slate-800">
            <div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 px-4 py-8 text-sm text-slate-500 dark:text-slate-400 sm:flex-row">
                <p>"© 2026 Azharul Islam · Built with Rust + Leptos"</p>
                <div class="flex items-center gap-4">
                    <a href="https://github.com/azharcse14" target="_blank" rel="noreferrer" class="hover:text-slate-900 dark:hover:text-slate-100">
                        "GitHub"
                    </a>
                    <A href="/contact" attr:class="hover:text-slate-900 dark:hover:text-slate-100">
                        "Contact"
                    </A>
                </div>
            </div>
        </footer>
    }
}
