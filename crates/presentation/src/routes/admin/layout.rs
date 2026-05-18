use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};

use crate::server_fns::auth::Logout;

#[component]
pub fn AdminShell(children: Children) -> impl IntoView {
    let logout = ServerAction::<Logout>::new();
    let location = use_location();

    let nav = use_navigate();
    Effect::new(move |_| {
        if matches!(logout.value().get(), Some(Ok(()))) {
            nav("/admin/login", Default::default());
        }
    });

    let is_active = move |path: &'static str| {
        let cur = location.pathname.get();
        if path == "/admin" {
            cur == "/admin" || cur == "/admin/"
        } else {
            cur.starts_with(path)
        }
    };

    view! {
        <div id="admin-app">
            <div class="admin-shell">
                <aside class="admin-sidebar">
                    <div class="brand">"⚡ Dashboard"</div>
                    <nav>
                        <A href="/admin"
                            attr:class=move || if is_active("/admin") && !location.pathname.get().contains("/admin/") { "active" } else { "" }>
                            "🏠 Home"
                        </A>
                        <A href="/admin/profile"
                            attr:class=move || if is_active("/admin/profile") { "active" } else { "" }>
                            "👤 Profile"
                        </A>
                        <A href="/admin/projects"
                            attr:class=move || if is_active("/admin/projects") { "active" } else { "" }>
                            "💼 Projects"
                        </A>
                        <A href="/admin/posts"
                            attr:class=move || if is_active("/admin/posts") { "active" } else { "" }>
                            "✍️  Blog"
                        </A>
                        <A href="/admin/skills"
                            attr:class=move || if is_active("/admin/skills") { "active" } else { "" }>
                            "🛠️  Skills"
                        </A>
                        <A href="/admin/messages"
                            attr:class=move || if is_active("/admin/messages") { "active" } else { "" }>
                            "📬 Messages"
                        </A>
                        <a href="/" target="_blank">"🌐 View public site →"</a>
                    </nav>
                    <div class="signout">
                        <ActionForm action=logout>
                            <input type="submit" class="btn btn-sm" value="Sign out" />
                        </ActionForm>
                    </div>
                </aside>
                <main class="admin-content">
                    {children()}
                </main>
            </div>
        </div>
    }
}
