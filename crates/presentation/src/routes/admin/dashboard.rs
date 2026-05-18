use leptos::prelude::*;
use leptos_meta::Title;

use crate::routes::admin::layout::AdminShell;
use crate::server_fns::admin::{list_all_posts, list_all_projects, list_all_skills, list_messages};

#[component]
pub fn AdminDashboardPage() -> impl IntoView {
    let projects = OnceResource::new(async { list_all_projects().await });
    let posts = OnceResource::new(async { list_all_posts().await });
    let skills = OnceResource::new(async { list_all_skills().await });
    let messages = OnceResource::new(async { list_messages().await });

    view! {
        <Title text="Admin · Dashboard" />
        <AdminShell>
            <div class="admin-header">
                <h1>"Dashboard"</h1>
            </div>

            <Suspense fallback=|| view! { <p>"Loading stats…"</p> }>
                {move || {
                    let p_count = projects.get().and_then(|r| r.ok()).map(|v| v.len()).unwrap_or(0);
                    let post_count = posts.get().and_then(|r| r.ok()).map(|v| v.len()).unwrap_or(0);
                    let skill_count = skills.get().and_then(|r| r.ok()).map(|v| v.len()).unwrap_or(0);
                    let msg_count = messages.get().and_then(|r| r.ok()).map(|v| v.len()).unwrap_or(0);
                    let unread = messages.get().and_then(|r| r.ok())
                        .map(|v| v.into_iter().filter(|m| !m.read).count())
                        .unwrap_or(0);
                    view! {
                        <div class="stats">
                            <div class="stat">
                                <div class="label">"Projects"</div>
                                <div class="value">{p_count}</div>
                            </div>
                            <div class="stat">
                                <div class="label">"Blog posts"</div>
                                <div class="value">{post_count}</div>
                            </div>
                            <div class="stat">
                                <div class="label">"Skills"</div>
                                <div class="value">{skill_count}</div>
                            </div>
                            <div class="stat">
                                <div class="label">"Messages"</div>
                                <div class="value">{msg_count}</div>
                            </div>
                            <div class="stat">
                                <div class="label">"Unread"</div>
                                <div class="value">{unread}</div>
                            </div>
                        </div>
                    }
                }}
            </Suspense>

            <div class="card">
                <h2>"Quick actions"</h2>
                <p>
                    <a class="btn btn-primary" href="/admin/projects/new">"+ New project"</a>" "
                    <a class="btn" href="/admin/posts/new">"+ New blog post"</a>" "
                    <a class="btn" href="/admin/skills">"+ Add skill"</a>" "
                    <a class="btn" href="/admin/profile">"Edit profile"</a>
                </p>
            </div>

            <div class="card">
                <h2>"Recent messages"</h2>
                <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                    {move || messages.get().map(|res| {
                        let items = res.unwrap_or_default();
                        if items.is_empty() {
                            return view! { <p>"No messages yet."</p> }.into_any();
                        }
                        let preview: Vec<_> = items.into_iter().take(5).collect();
                        view! {
                            <table>
                                <thead>
                                    <tr><th>"From"</th><th>"Subject"</th><th>"Received"</th><th></th></tr>
                                </thead>
                                <tbody>
                                    {preview.into_iter().map(|m| view! {
                                        <tr>
                                            <td>
                                                <strong>{m.name}</strong>
                                                <br />
                                                <small>{m.email}</small>
                                            </td>
                                            <td>
                                                {if m.read { view! {}.into_any() } else { view! {
                                                    <span class="badge badge-warning">"New"</span>" "
                                                }.into_any() }}
                                                {if m.subject.is_empty() { "(no subject)".to_string() } else { m.subject }}
                                            </td>
                                            <td><small>{m.created_at}</small></td>
                                            <td><a href="/admin/messages">"Open →"</a></td>
                                        </tr>
                                    }).collect_view()}
                                </tbody>
                            </table>
                        }.into_any()
                    })}
                </Suspense>
            </div>
        </AdminShell>
    }
}
