use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_navigate;

use crate::server_fns::auth::{admin_setup_open, Login, RegisterAdmin};

#[component]
pub fn LoginPage() -> impl IntoView {
    let action = ServerAction::<Login>::new();
    let value = action.value();
    let pending = action.pending();

    // Redirect to dashboard on successful login.
    let nav = use_navigate();
    Effect::new(move |_| {
        if matches!(value.get(), Some(Ok(()))) {
            nav("/admin", Default::default());
        }
    });

    view! {
        <Title text="Admin · Sign In" />
        <Meta name="robots" content="noindex, nofollow" />
        <div id="admin-app">
            <div class="auth-wrap">
                <div class="auth-card">
                    <h1>"Sign in"</h1>
                    <p class="hint">"Welcome back — sign in to manage your portfolio."</p>

                    {move || match value.get() {
                        Some(Err(e)) => view! {
                            <div class="alert alert-error">
                                {format!("Couldn't sign in: {}", error_message(&e))}
                            </div>
                        }.into_any(),
                        _ => view! {}.into_any(),
                    }}

                    <ActionForm action=action>
                        <div class="form-group">
                            <label for="email">"Email"</label>
                            <input type="email" id="email" name="email" required />
                        </div>
                        <div class="form-group">
                            <label for="password">"Password"</label>
                            <input type="password" id="password" name="password" required />
                        </div>
                        <input
                            type="submit"
                            class="btn btn-primary"
                            style="width:100%;"
                            value=move || if pending.get() { "Signing in…" } else { "Sign in" }
                        />
                    </ActionForm>

                    <p class="hint" style="margin-top:24px;text-align:center;">
                        "First time? "
                        <a href="/admin/setup">"Run setup →"</a>
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SetupPage() -> impl IntoView {
    let setup_open = OnceResource::new(async { admin_setup_open().await.unwrap_or(false) });
    let action = ServerAction::<RegisterAdmin>::new();
    let value = action.value();
    let pending = action.pending();

    let nav = use_navigate();
    Effect::new(move |_| {
        if matches!(value.get(), Some(Ok(()))) {
            nav("/admin", Default::default());
        }
    });

    view! {
        <Title text="Admin · Setup" />
        <Meta name="robots" content="noindex, nofollow" />
        <div id="admin-app">
            <div class="auth-wrap">
                <div class="auth-card">
                    <Suspense fallback=|| view! { <p class="hint">"Loading…"</p> }>
                        {move || setup_open.get().map(|open| {
                            if !open {
                                return view! {
                                    <h1>"Setup already complete"</h1>
                                    <p class="hint">
                                        "An admin account already exists. "
                                        <a href="/admin/login">"Sign in →"</a>
                                    </p>
                                }.into_any();
                            }
                            view! {
                                <h1>"Create admin"</h1>
                                <p class="hint">"This is a one-time bootstrap to create the first admin account."</p>

                                {move || match value.get() {
                                    Some(Err(e)) => view! {
                                        <div class="alert alert-error">
                                            {format!("Couldn't create: {}", error_message(&e))}
                                        </div>
                                    }.into_any(),
                                    _ => view! {}.into_any(),
                                }}

                                <ActionForm action=action>
                                    <div class="form-group">
                                        <label for="name">"Name"</label>
                                        <input type="text" id="name" name="name" />
                                    </div>
                                    <div class="form-group">
                                        <label for="email">"Email"</label>
                                        <input type="email" id="email" name="email" required />
                                    </div>
                                    <div class="form-group">
                                        <label for="password">"Password (min 8 chars)"</label>
                                        <input type="password" id="password" name="password" minlength="8" required />
                                    </div>
                                    <input
                                        type="submit"
                                        class="btn btn-primary"
                                        style="width:100%;"
                                        value=move || if pending.get() { "Creating…" } else { "Create admin" }
                                    />
                                </ActionForm>
                            }.into_any()
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

fn error_message(e: &leptos::server_fn::ServerFnError) -> String {
    let s = e.to_string();
    // Strip generic "error running server function: " noise.
    s.split(": ").last().unwrap_or(&s).to_string()
}
