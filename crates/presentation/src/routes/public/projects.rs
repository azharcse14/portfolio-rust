use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{ProjectCard, SectionHeading};
use crate::data::PROJECTS;
use crate::layouts::PublicLayout;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <Title text="Projects — Azharul Islam" />
        <PublicLayout>
            <section class="py-12">
                <SectionHeading
                    eyebrow="Work"
                    title="All projects"
                    subtitle="The full archive — featured work, side projects, and occasional experiments."
                />
                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    {PROJECTS.iter().map(|p| view! { <ProjectCard project=p /> }).collect_view()}
                </div>
            </section>
        </PublicLayout>
    }
}
