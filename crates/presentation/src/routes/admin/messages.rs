use leptos::prelude::*;
use leptos_meta::Title;

use crate::routes::admin::layout::AdminShell;
use crate::server_fns::admin::{list_messages, DeleteMessage, MarkMessageRead};

#[component]
pub fn AdminMessagesPage() -> impl IntoView {
    let mark_read = ServerAction::<MarkMessageRead>::new();
    let delete = ServerAction::<DeleteMessage>::new();
    let version = move || (mark_read.version().get(), delete.version().get());
    let messages = Resource::new(version, |_| async { list_messages().await });

    view! {
        <Title text="Admin · Messages" />
        <AdminShell>
            <div class="admin-header">
                <h1>"Messages"</h1>
            </div>

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || messages.get().map(|res| {
                    let items = res.unwrap_or_default();
                    if items.is_empty() {
                        return view! { <p>"No messages yet."</p> }.into_any();
                    }
                    view! {
                        <div>
                            {items.into_iter().map(|m| {
                                let id_for_read = m.id.clone();
                                let id_for_delete = m.id.clone();
                                view! {
                                    <div class="card">
                                        <div style="display:flex;justify-content:space-between;align-items:flex-start;gap:16px;">
                                            <div style="flex:1;">
                                                <h3 style="margin-bottom:4px;">
                                                    {if !m.read { view! { <span class="badge badge-warning">"NEW"</span> }.into_any() } else { view! {}.into_any() }}
                                                    " "
                                                    {if m.subject.is_empty() { "(no subject)".to_string() } else { m.subject.clone() }}
                                                </h3>
                                                <p style="margin:0;color:#6b7280;font-size:13px;">
                                                    "From "<strong>{m.name.clone()}</strong>
                                                    " <"<a href=format!("mailto:{}", m.email)>{m.email.clone()}</a>">"
                                                    " · "{m.created_at.clone()}
                                                </p>
                                            </div>
                                            <div style="display:flex;gap:8px;">
                                                {if !m.read {
                                                    view! {
                                                        <ActionForm action=mark_read attr:style="display:inline;">
                                                            <input type="hidden" name="id" value=id_for_read />
                                                            <input type="submit" class="btn btn-sm" value="Mark read" />
                                                        </ActionForm>
                                                    }.into_any()
                                                } else { view! {}.into_any() }}
                                                <ActionForm action=delete attr:style="display:inline;">
                                                    <input type="hidden" name="id" value=id_for_delete />
                                                    <input type="submit" class="btn btn-sm btn-danger" value="Delete"
                                                        onclick="return confirm('Delete this message?')" />
                                                </ActionForm>
                                            </div>
                                        </div>
                                        <div style="margin-top:12px;padding-top:12px;border-top:1px solid #e5e7eb;white-space:pre-wrap;">
                                            {m.body.clone()}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </AdminShell>
    }
}
