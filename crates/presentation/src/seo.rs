//! SEO helpers — sitemap generation served as a plain Axum handler so that
//! crawlers see proper XML content-type (not HTML wrapper from Leptos SSR).

#[cfg(feature = "ssr")]
pub async fn sitemap_xml(state: crate::state::AppState) -> impl axum::response::IntoResponse {
    use axum::http::{header, StatusCode};

    let posts = state.list_posts.execute().await.unwrap_or_default();
    let projects = state.list_projects.execute(false).await.unwrap_or_default();

    let mut xml = String::with_capacity(2048);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    // Home — highest priority
    xml.push_str("  <url>\n");
    xml.push_str("    <loc>/</loc>\n");
    xml.push_str("    <changefreq>weekly</changefreq>\n");
    xml.push_str("    <priority>1.0</priority>\n");
    xml.push_str("  </url>\n");

    // Project anchors (same page, useful for crawl signals)
    for p in &projects {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>/#work-{}</loc>\n", p.slug.as_str()));
        xml.push_str("    <priority>0.7</priority>\n");
        xml.push_str("  </url>\n");
    }

    // Blog post anchors
    for post in &posts {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>/#blog-{}</loc>\n", post.slug.as_str()));
        if let Some(d) = post.published_at {
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", d.format("%Y-%m-%d")));
        }
        xml.push_str("    <priority>0.6</priority>\n");
        xml.push_str("  </url>\n");
    }

    xml.push_str("</urlset>\n");

    (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            "application/xml; charset=utf-8",
        )],
        xml,
    )
}
