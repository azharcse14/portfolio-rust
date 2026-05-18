use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::routes::admin::layout::AdminShell;
use crate::server_fns::admin::{
    get_post_for_edit, list_all_posts, DeletePost, PostForm, SavePost,
};

#[component]
pub fn AdminPostsPage() -> impl IntoView {
    let delete = ServerAction::<DeletePost>::new();
    let version = delete.version();
    let posts = Resource::new(move || version.get(), |_| async { list_all_posts().await });

    view! {
        <Title text="Admin · Blog" />
        <AdminShell>
            <div class="admin-header">
                <h1>"Blog posts"</h1>
                <a class="btn btn-primary" href="/admin/posts/new">"+ New post"</a>
            </div>

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || posts.get().map(|res| {
                    let items = res.unwrap_or_default();
                    if items.is_empty() {
                        return view! { <p>"No posts yet."</p> }.into_any();
                    }
                    view! {
                        <table>
                            <thead>
                                <tr><th>"Title"</th><th>"Slug"</th><th>"Status"</th><th></th></tr>
                            </thead>
                            <tbody>
                                {items.into_iter().map(|p| {
                                    let edit_href = format!("/admin/posts/{}", p.slug);
                                    view! {
                                        <tr>
                                            <td><strong>{p.title}</strong></td>
                                            <td><code>{p.slug}</code></td>
                                            <td>
                                                {if p.published {
                                                    view! { <span class="badge badge-success">"Published"</span> }.into_any()
                                                } else {
                                                    view! { <span class="badge badge-warning">"Draft"</span> }.into_any()
                                                }}
                                            </td>
                                            <td style="text-align:right;">
                                                <a class="btn btn-sm" href=edit_href>"Edit"</a>" "
                                                <ActionForm action=delete attr:style="display:inline;">
                                                    <input type="hidden" name="id" value=p.id />
                                                    <input type="submit" class="btn btn-sm btn-danger" value="Delete"
                                                        onclick="return confirm('Delete this post?')" />
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
pub fn AdminPostEditPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    let is_new = move || slug() == "new";

    let save = ServerAction::<SavePost>::new();
    let value = save.value();

    let existing = OnceResource::new({
        let slug = slug();
        let is_new = is_new();
        async move {
            if is_new {
                return Ok(blank_post());
            }
            get_post_for_edit(slug).await
        }
    });

    let nav = use_navigate();
    Effect::new(move |_| {
        if matches!(value.get(), Some(Ok(()))) {
            nav("/admin/posts", Default::default());
        }
    });

    view! {
        <Title text="Admin · Edit post" />
        <AdminShell>
            <div class="admin-header">
                <h1>{move || if is_new() { "New post" } else { "Edit post" }}</h1>
                <a class="btn" href="/admin/posts">"← Back"</a>
            </div>

            {move || match value.get() {
                Some(Err(e)) => view! { <div class="alert alert-error">{format!("Save failed: {e}")}</div> }.into_any(),
                _ => view! {}.into_any(),
            }}

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || existing.get().map(|res| {
                    let p = res.unwrap_or_else(|_| blank_post());
                    view! {
                        <PostFormView form=p slug=slug() is_new=is_new() save_action=save />
                    }
                })}
            </Suspense>
        </AdminShell>
    }
}

fn blank_post() -> PostForm {
    PostForm {
        title: String::new(),
        slug: String::new(),
        content_md: String::new(),
        cover_image: String::new(),
        tags: String::new(),
        published: false,
    }
}

#[component]
fn PostFormView(
    form: PostForm,
    slug: String,
    is_new: bool,
    save_action: ServerAction<SavePost>,
) -> impl IntoView {
    let id_resource = OnceResource::new(async move {
        if is_new {
            return String::new();
        }
        let list = list_all_posts().await.unwrap_or_default();
        list.into_iter()
            .find(|p| p.slug == slug)
            .map(|p| p.id)
            .unwrap_or_default()
    });

    let (md, set_md) = signal(form.content_md.clone());
    let preview = move || render_md(&md.get());
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
                                    <label>"Slug"</label>
                                    <input type="text" name="form[slug]" value=f.slug required />
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label>"Cover image URL"</label>
                                    <input type="text" name="form[cover_image]" value=f.cover_image placeholder="/images/blog-1.jpg" />
                                </div>
                                <div class="form-group">
                                    <label>"Tags (comma-separated)"</label>
                                    <input type="text" name="form[tags]" value=f.tags placeholder="Rust, Leptos" />
                                </div>
                            </div>

                            <div class="form-group">
                                <label>"Content (Markdown) — live preview on right"</label>
                                <div class="md-editor">
                                    <textarea
                                        name="form[content_md]"
                                        style="min-height:500px;"
                                        prop:value=md
                                        on:input=move |ev| set_md.set(event_target_value(&ev))
                                    ></textarea>
                                    <div class="md-preview" inner_html=preview></div>
                                </div>
                            </div>

                            <div class="form-checkbox">
                                <input type="checkbox" name="form[published]" id="pub" value="true" checked=f.published />
                                <label for="pub">"Published"</label>
                            </div>

                            <div class="form-actions">
                                <input type="submit" class="btn btn-primary" value="Save post" />
                                <a class="btn" href="/admin/posts">"Cancel"</a>
                            </div>
                        </ActionForm>
                    }
                })}
            </Suspense>
        </div>
    }
}

fn render_md(md: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(
        md,
        pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_TASKLISTS,
    );
    let mut out = String::new();
    pulldown_cmark::html::push_html(&mut out, parser);
    out
}
