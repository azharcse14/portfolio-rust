#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use infrastructure::{db::connect_and_migrate, AppConfig};
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use presentation::{
        app::{shell, App},
        state::AppState,
    };
    use tower_http::services::ServeDir;
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
    let state = AppState::build(pool);

    let conf = get_configuration(None).expect("failed to load Leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
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
        .fallback_service(ServeDir::new(&*leptos_options.site_root))
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
