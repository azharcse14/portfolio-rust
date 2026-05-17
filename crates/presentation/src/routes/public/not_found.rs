use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(response_options) = use_context::<leptos_axum::ResponseOptions>() {
            response_options.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <Title text="Page not found — Azharul Islam" />
        <div id="colorlib-page">
            <div class="container-wrap">
                <div id="colorlib-main">
                    <section style="padding: 120px 40px; text-align: center;">
                        <h1 style="font-size: 96px; margin: 0; color: #ccc;">"404"</h1>
                        <h2 class="colorlib-heading">"Page not found"</h2>
                        <p>"The page you're looking for doesn't exist."</p>
                        <p>
                            <a href="/" class="btn btn-primary btn-learn">"← Back home"</a>
                        </p>
                    </section>
                </div>
            </div>
        </div>
    }
}
