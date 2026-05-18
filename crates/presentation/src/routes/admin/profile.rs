use leptos::prelude::*;
use leptos_meta::Title;

use crate::routes::admin::layout::AdminShell;
use crate::server_fns::admin::{get_profile_form, ProfileForm, UpdateProfile};

#[component]
pub fn AdminProfilePage() -> impl IntoView {
    let profile = OnceResource::new(async { get_profile_form().await });
    let action = ServerAction::<UpdateProfile>::new();
    let value = action.value();

    view! {
        <Title text="Admin · Profile" />
        <AdminShell>
            <div class="admin-header">
                <h1>"Profile"</h1>
            </div>

            {move || match value.get() {
                Some(Ok(())) => view! { <div class="alert alert-success">"✓ Profile saved."</div> }.into_any(),
                Some(Err(e)) => view! { <div class="alert alert-error">{format!("Save failed: {e}")}</div> }.into_any(),
                None => view! {}.into_any(),
            }}

            <Suspense fallback=|| view! { <p>"Loading…"</p> }>
                {move || profile.get().map(|res| {
                    let p = res.unwrap_or_else(|_| ProfileForm {
                        name: String::new(), title: String::new(), bio: String::new(),
                        photo_url: String::new(), email: String::new(), github: String::new(),
                        linkedin: String::new(), twitter: String::new(),
                    });
                    view! {
                        <div class="card">
                            <ActionForm action=action>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>"Name"</label>
                                        <input type="text" name="form[name]" value=p.name required />
                                    </div>
                                    <div class="form-group">
                                        <label>"Title"</label>
                                        <input type="text" name="form[title]" value=p.title required />
                                    </div>
                                </div>
                                <div class="form-group">
                                    <label>"Bio"</label>
                                    <textarea name="form[bio]">{p.bio}</textarea>
                                </div>
                                <div class="form-group">
                                    <label>"Photo URL"</label>
                                    <input type="text" name="form[photo_url]" value=p.photo_url placeholder="/images/about.jpg" />
                                </div>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>"Email"</label>
                                        <input type="email" name="form[email]" value=p.email />
                                    </div>
                                    <div class="form-group">
                                        <label>"GitHub"</label>
                                        <input type="url" name="form[github]" value=p.github />
                                    </div>
                                </div>
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>"LinkedIn"</label>
                                        <input type="url" name="form[linkedin]" value=p.linkedin />
                                    </div>
                                    <div class="form-group">
                                        <label>"Twitter"</label>
                                        <input type="url" name="form[twitter]" value=p.twitter />
                                    </div>
                                </div>
                                <div class="form-actions">
                                    <input type="submit" class="btn btn-primary" value="Save profile" />
                                </div>
                            </ActionForm>
                        </div>
                    }
                })}
            </Suspense>
        </AdminShell>
    }
}
