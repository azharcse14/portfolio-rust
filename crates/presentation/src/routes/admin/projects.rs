use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::routes::admin::layout::AdminShell;
use crate::server_fns::admin::{
    get_project_for_edit, list_all_projects, DeleteProject, ProjectForm, SaveProject,
};

#[component]
pub fn AdminProjectsPage() -> impl IntoView {
    let delete = ServerAction::<DeleteProject>::new();
    let version = delete.version();
    let projects = Resource::new(move || version.get(), |_| async { list_all_projects().await });

    view! {
        <Title text="Admin · Projects" />
        <AdminShell>
            <div class="admin-header">
                <h1>"Projects"</h1>
                <a class="btn btn-primary" href="/admin/projects/new">"+ New project"</a>
            </div>

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || projects.get().map(|res| {
                    let items = res.unwrap_or_default();
                    if items.is_empty() {
                        return view! { <p>"No projects yet. Add your first one above."</p> }.into_any();
                    }
                    view! {
                        <table>
                            <thead>
                                <tr><th>"Title"</th><th>"Slug"</th><th>"Featured"</th><th>"Order"</th><th></th></tr>
                            </thead>
                            <tbody>
                                {items.into_iter().map(|p| {
                                    let edit_href = format!("/admin/projects/{}", p.slug);
                                    let id_for_delete = p.id.clone();
                                    view! {
                                        <tr>
                                            <td><strong>{p.title}</strong></td>
                                            <td><code>{p.slug}</code></td>
                                            <td>
                                                {if p.featured {
                                                    view! { <span class="badge badge-success">"★ Yes"</span> }.into_any()
                                                } else {
                                                    view! { <span class="badge">"No"</span> }.into_any()
                                                }}
                                            </td>
                                            <td>{p.display_order}</td>
                                            <td style="text-align:right;">
                                                <a class="btn btn-sm" href=edit_href>"Edit"</a>" "
                                                <ActionForm action=delete attr:style="display:inline;">
                                                    <input type="hidden" name="id" value=id_for_delete />
                                                    <input type="submit" class="btn btn-sm btn-danger" value="Delete"
                                                        onclick="return confirm('Delete this project?')" />
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    }.into_any()
                })}
            </Suspense>
        </AdminShell>
    }
}

#[component]
pub fn AdminProjectEditPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    let is_new = move || slug() == "new";

    let save = ServerAction::<SaveProject>::new();
    let value = save.value();

    // Load existing data if editing.
    let existing = OnceResource::new({
        let slug = slug();
        let is_new = is_new();
        async move {
            if is_new {
                return Ok(blank_project());
            }
            get_project_for_edit(slug).await
        }
    });

    let nav = use_navigate();
    Effect::new(move |_| {
        if matches!(value.get(), Some(Ok(()))) {
            nav("/admin/projects", Default::default());
        }
    });

    view! {
        <Title text="Admin · Edit project" />
        <AdminShell>
            <div class="admin-header">
                <h1>{move || if is_new() { "New project" } else { "Edit project" }}</h1>
                <a class="btn" href="/admin/projects">"← Back"</a>
            </div>

            {move || match value.get() {
                Some(Err(e)) => view! { <div class="alert alert-error">{format!("Save failed: {e}")}</div> }.into_any(),
                _ => view! {}.into_any(),
            }}

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || existing.get().map(|res| {
                    let p = res.unwrap_or_else(|_| blank_project());
                    let id = if is_new() { String::new() } else {
                        // We don't ship the id over edit URL — re-fetch via list lookup.
                        // For brevity here, server side will resolve slug -> uuid via update path.
                        // Workaround: pass empty id to create OR look up id via list_all_projects.
                        // We re-resolve below via an inner resource.
                        String::new()
                    };
                    let _ = id;
                    view! {
                        <ProjectForm form=p slug=slug() is_new=is_new() save_action=save />
                    }
                })}
            </Suspense>
        </AdminShell>
    }
}

fn blank_project() -> ProjectForm {
    ProjectForm {
        title: String::new(),
        slug: String::new(),
        description: String::new(),
        image_url: String::new(),
        tech_stack: String::new(),
        github_url: String::new(),
        live_url: String::new(),
        featured: false,
        display_order: 0,
    }
}

#[component]
fn ProjectForm(
    form: ProjectForm,
    slug: String,
    is_new: bool,
    save_action: ServerAction<SaveProject>,
) -> impl IntoView {
    // For editing, resolve existing id by looking up the project list once.
    let id_resource = OnceResource::new(async move {
        if is_new {
            return String::new();
        }
        let list = list_all_projects().await.unwrap_or_default();
        list.into_iter()
            .find(|p| p.slug == slug)
            .map(|p| p.id)
            .unwrap_or_default()
    });

    let form = StoredValue::new(form);
    view! {
        <div class="card">
            <Suspense fallback=|| view! { <p>"…"</p> }>
                {move || id_resource.get().map(|id| {
                    let f = form.get_value();
                    view! {
                        <ActionForm action=save_action>
                            <input type="hidden" name="id" value=id />
                            <div class="form-row">
                                <div class="form-group">
                                    <label>"Title"</label>
                                    <input type="text" name="form[title]" value=f.title required />
                                </div>
                                <div class="form-group">
                                    <label>"Slug (URL-safe)"</label>
                                    <input type="text" name="form[slug]" value=f.slug required />
                                </div>
                            </div>
                            <div class="form-group">
                                <label>"Description"</label>
                                <textarea name="form[description]" required>{f.description}</textarea>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label>"Image URL"</label>
                                    <input type="text" name="form[image_url]" value=f.image_url placeholder="/images/img-1.jpg" />
                                </div>
                                <div class="form-group">
                                    <label>"Tech stack (comma-separated)"</label>
                                    <input type="text" name="form[tech_stack]" value=f.tech_stack placeholder="Rust, Axum, SQLx" />
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label>"GitHub URL"</label>
                                    <input type="url" name="form[github_url]" value=f.github_url />
                                </div>
                                <div class="form-group">
                                    <label>"Live URL"</label>
                                    <input type="url" name="form[live_url]" value=f.live_url />
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-checkbox">
                                    <input type="checkbox" name="form[featured]" id="featured" value="true" checked=f.featured />
                                    <label for="featured">"Featured"</label>
                                </div>
                                <div class="form-group">
                                    <label>"Display order"</label>
                                    <input type="number" name="form[display_order]" value=f.display_order.to_string() />
                                </div>
                            </div>
                            <div class="form-actions">
                                <input type="submit" class="btn btn-primary" value="Save project" />
                                <a class="btn" href="/admin/projects">"Cancel"</a>
                            </div>
                        </ActionForm>
                    }
                })}
            </Suspense>
        </div>
    }
}
