//! Presentation layer — Leptos components, routes, server functions.
//! Compiled twice: once as cdylib for browser hydration, once linked into the server bin.

pub mod app;
pub mod components;
pub mod layouts;
pub mod routes;
pub mod server_fns;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
