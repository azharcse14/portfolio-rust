use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-24 border-t border-slate-200 py-8 text-center text-sm text-slate-500 dark:border-slate-800 dark:text-slate-400">
            <p>"© 2026 Azharul Islam · Built with Rust + Leptos"</p>
        </footer>
    }
}
