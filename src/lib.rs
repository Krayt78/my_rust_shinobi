//! My Rust Shinobi - A ninja-themed web game built with Leptos

pub mod api;
pub mod app;
pub mod components;
pub mod db;
pub mod pages;
pub mod wallet;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
