use leptos::prelude::*;
use leptos::server_fn::ServerFnError;

pub const COOKIE_NAME: &str = "auth_token";

#[server]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    let token = state.login.execute(&email, &password).await?;
    set_auth_cookie(&token)?;
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    set_cookie_raw(&format!(
        "{COOKIE_NAME}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"
    ))
}

#[server]
pub async fn register_admin(
    email: String,
    password: String,
    name: String,
) -> Result<(), ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    let name = if name.trim().is_empty() {
        None
    } else {
        Some(name)
    };
    let user_id = state.register_admin.execute(&email, &password, name).await?;
    let token = state.tokens.issue(user_id, 24 * 7)?;
    set_auth_cookie(&token)?;
    Ok(())
}

#[server]
pub async fn admin_setup_open() -> Result<bool, ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    Ok(!state.is_setup_complete.execute().await?)
}

#[server]
pub async fn current_admin() -> Result<Option<String>, ServerFnError> {
    Ok(current_admin_id_inner().await.ok().map(|id| id.to_string()))
}

#[cfg(feature = "ssr")]
fn set_auth_cookie(token: &str) -> Result<(), ServerFnError> {
    set_cookie_raw(&format!(
        "{COOKIE_NAME}={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=604800"
    ))
}

#[cfg(not(feature = "ssr"))]
fn set_auth_cookie(_token: &str) -> Result<(), ServerFnError> {
    Ok(())
}

#[cfg(feature = "ssr")]
fn set_cookie_raw(cookie: &str) -> Result<(), ServerFnError> {
    let resp = expect_context::<leptos_axum::ResponseOptions>();
    let value: axum::http::HeaderValue = cookie.parse()?;
    resp.insert_header(axum::http::header::SET_COOKIE, value);
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn set_cookie_raw(_cookie: &str) -> Result<(), ServerFnError> {
    Ok(())
}

/// Helper used by admin-only server fns to enforce a valid auth cookie.
/// Reads `auth_token` cookie, verifies via `TokenIssuer`, returns the user id.
#[cfg(feature = "ssr")]
pub(crate) async fn require_admin() -> Result<uuid::Uuid, ServerFnError> {
    current_admin_id_inner()
        .await
        .map_err(|_| ServerFnError::ServerError("unauthorized".into()))
}

#[cfg(feature = "ssr")]
async fn current_admin_id_inner() -> Result<uuid::Uuid, String> {
    use axum::http::HeaderMap;
    let state = expect_context::<crate::state::AppState>();
    let headers: HeaderMap = leptos_axum::extract()
        .await
        .map_err(|e: ServerFnError| e.to_string())?;
    let cookie_header = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let token = cookie_header
        .split(';')
        .map(|p| p.trim())
        .find_map(|p| p.strip_prefix(&format!("{COOKIE_NAME}=")))
        .ok_or_else(|| "no token".to_string())?;
    state.tokens.verify(token).map_err(|e| e.to_string())
}

#[cfg(not(feature = "ssr"))]
async fn current_admin_id_inner() -> Result<uuid::Uuid, String> {
    Err("ssr only".into())
}
