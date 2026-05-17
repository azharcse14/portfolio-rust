use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{BlogCard, SectionHeading};
use crate::data::POSTS;
use crate::layouts::PublicLayout;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <Title text="Blog — Azharul Islam" />
        <PublicLayout>
            <section class="py-12">
                <SectionHeading
                    eyebrow="Writing"
                    title="Blog"
                    subtitle="Notes on Rust, frontend, architecture, and whatever I've been learning lately."
                />
                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
                    {POSTS.iter().map(|p| view! { <BlogCard post=p /> }).collect_view()}
                </div>
            </section>
        </PublicLayout>
    }
}
