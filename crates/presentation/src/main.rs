#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use axum::{
        http::{header, HeaderValue},
        Router,
    };
    use infrastructure::{db::connect_and_migrate, AppConfig};
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use presentation::{
        app::{shell, App},
        seo::sitemap_xml,
        state::AppState,
    };
    use tower::ServiceBuilder;
    use tower_http::{
        compression::CompressionLayer,
        services::ServeDir,
        set_header::SetResponseHeaderLayer,
    };
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = AppConfig::from_env();
    tracing::info!("connecting to db: {}", cfg.database_url);
    let pool = connect_and_migrate(&cfg.database_url)
        .await
        .expect("db connect + migrate");
    let state = AppState::build(pool, &cfg);

    let conf = get_configuration(None).expect("failed to load Leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Long-lived cache for static asset directories (cargo-leptos hashes /pkg
    // bundle names, and our /images, /css, /js, /fonts are versioned by content).
    let one_year = Duration::from_secs(60 * 60 * 24 * 365);
    let immutable_assets = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_str(&format!(
            "public, max-age={}, immutable",
            one_year.as_secs()
        ))
        .unwrap(),
    );

    let static_serve = ServeDir::new(&*leptos_options.site_root)
        .precompressed_gzip()
        .precompressed_br();
    // Cache-Control: only applied to the static fallback so HTML stays fresh.
    let cached_static = ServiceBuilder::new()
        .layer(immutable_assets)
        .service(static_serve);

    let app = Router::new()
        // SEO route — must be before the Leptos catch-all
        .route(
            "/sitemap.xml",
            axum::routing::get({
                let state = state.clone();
                move || {
                    let state = state.clone();
                    async move { sitemap_xml(state).await }
                }
            }),
        )
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let state = state.clone();
                move || provide_context(state.clone())
            },
            {
                let opts = leptos_options.clone();
                move || shell(opts.clone())
            },
        )
        .fallback_service(cached_static)
        .layer(CompressionLayer::new())
        .with_state(leptos_options);

    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // No-op: cdylib build doesn't run main; hydration entry is in lib.rs.
}
