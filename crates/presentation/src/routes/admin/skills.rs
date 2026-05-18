use leptos::prelude::*;
use leptos_meta::Title;

use crate::routes::admin::layout::AdminShell;
use crate::server_fns::admin::{list_all_skills, AddSkill, DeleteSkill};

#[component]
pub fn AdminSkillsPage() -> impl IntoView {
    let add = ServerAction::<AddSkill>::new();
    let delete = ServerAction::<DeleteSkill>::new();
    let version = move || (add.version().get(), delete.version().get());
    let skills = Resource::new(version, |_| async { list_all_skills().await });

    view! {
        <Title text="Admin · Skills" />
        <AdminShell>
            <div class="admin-header">
                <h1>"Skills"</h1>
            </div>

            <div class="card">
                <h2>"Add a skill"</h2>
                <ActionForm action=add>
                    <div class="form-row">
                        <div class="form-group">
                            <label>"Name"</label>
                            <input type="text" name="name" required />
                        </div>
                        <div class="form-group">
                            <label>"Percentage (0–100)"</label>
                            <input type="number" name="percentage" min="0" max="100" value="80" required />
                        </div>
                        <div class="form-group">
                            <label>"Color"</label>
                            <select name="color">
                                <option value="color-1">"color-1 (blue)"</option>
                                <option value="color-2">"color-2 (red)"</option>
                                <option value="color-3">"color-3 (yellow)"</option>
                                <option value="color-4">"color-4 (green)"</option>
                                <option value="color-5">"color-5 (purple)"</option>
                                <option value="color-6">"color-6 (orange)"</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label>"Category"</label>
                            <input type="text" name="category" placeholder="Language" />
                        </div>
                        <div class="form-group">
                            <label>"Order"</label>
                            <input type="number" name="display_order" value="0" />
                        </div>
                    </div>
                    <div class="form-actions">
                        <input type="submit" class="btn btn-primary" value="Add skill" />
                    </div>
                </ActionForm>
            </div>

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || skills.get().map(|res| {
                    let items = res.unwrap_or_default();
                    if items.is_empty() {
                        return view! { <p>"No skills yet."</p> }.into_any();
                    }
                    view! {
                        <table>
                            <thead>
                                <tr><th>"Name"</th><th>"Category"</th><th>"%"</th><th>"Color"</th><th>"Order"</th><th></th></tr>
                            </thead>
                            <tbody>
                                {items.into_iter().map(|s| view! {
                                    <tr>
                                        <td><strong>{s.name}</strong></td>
                                        <td>{s.category}</td>
                                        <td>{s.percentage}"%"</td>
                                        <td><code>{s.color}</code></td>
                                        <td>{s.display_order}</td>
                                        <td style="text-align:right;">
                                            <ActionForm action=delete attr:style="display:inline;">
                                                <input type="hidden" name="id" value=s.id />
                                                <input type="submit" class="btn btn-sm btn-danger" value="Delete"
                                                    onclick="return confirm('Delete this skill?')" />
                                            </ActionForm>
                                        </td>
                                    </tr>
                                }).collect_view()}
                            </tbody>
                        </table>
                    }.into_any()
                })}
            </Suspense>
        </AdminShell>
    }
}
