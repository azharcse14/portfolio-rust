use leptos::prelude::*;

use crate::components::{Footer, Navbar};

#[component]
pub fn PublicLayout(children: Children) -> impl IntoView {
    view! {
        <div class="flex min-h-screen flex-col">
            <Navbar />
            <main class="mx-auto w-full max-w-6xl flex-1 px-4 py-12">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
