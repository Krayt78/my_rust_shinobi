mod components;
pub mod context;

pub use components::*;
pub use context::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct WalletAccount {
    pub address: String,
    pub name: Option<String>,
    pub source: String,
}

/// Connect to the Polkadot wallet extension
#[cfg(feature = "hydrate")]
pub async fn connect_polkadot_wallet(app_name: &str) -> Result<Vec<WalletAccount>, String> {
    use js_sys::{Promise, Reflect};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or("No window object")?;

    // Get the polkadotWallet global object
    let wallet_obj = Reflect::get(&window, &JsValue::from_str("polkadotWallet"))
        .map_err(|_| "polkadotWallet not found. Make sure wallet.js is loaded.")?;

    if wallet_obj.is_undefined() {
        return Err("polkadotWallet not found. Make sure wallet.js is loaded.".to_string());
    }

    // Get the connectWallet function
    let connect_fn = Reflect::get(&wallet_obj, &JsValue::from_str("connectWallet"))
        .map_err(|_| "connectWallet function not found")?;

    let connect_fn: js_sys::Function = connect_fn
        .dyn_into()
        .map_err(|_| "connectWallet is not a function")?;

    // Call the function with the app name
    let promise = connect_fn
        .call1(&wallet_obj, &JsValue::from_str(app_name))
        .map_err(|e| format!("Failed to call connectWallet: {:?}", e))?;

    let promise: Promise = promise
        .dyn_into()
        .map_err(|_| "connectWallet did not return a Promise")?;

    // Await the promise
    let result = JsFuture::from(promise)
        .await
        .map_err(|e| format!("Wallet connection failed: {:?}", e))?;

    // Parse the result
    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Failed to parse accounts: {:?}", e))
}

/// Check if wallet is available
#[cfg(feature = "hydrate")]
pub async fn check_wallet_available() -> bool {
    use js_sys::{Promise, Reflect};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };

    let wallet_obj = match Reflect::get(&window, &JsValue::from_str("polkadotWallet")) {
        Ok(obj) if !obj.is_undefined() => obj,
        _ => return false,
    };

    let is_available_fn = match Reflect::get(&wallet_obj, &JsValue::from_str("isWalletAvailable")) {
        Ok(f) => f,
        _ => return false,
    };

    let is_available_fn: js_sys::Function = match is_available_fn.dyn_into() {
        Ok(f) => f,
        _ => return false,
    };

    let promise = match is_available_fn.call0(&wallet_obj) {
        Ok(p) => p,
        _ => return false,
    };

    let promise: Promise = match promise.dyn_into() {
        Ok(p) => p,
        _ => return false,
    };

    match JsFuture::from(promise).await {
        Ok(val) => val.as_bool().unwrap_or(false),
        _ => false,
    }
}

// Stub implementations for SSR
#[cfg(not(feature = "hydrate"))]
pub async fn connect_polkadot_wallet(_app_name: &str) -> Result<Vec<WalletAccount>, String> {
    Err("Wallet connection only available in browser".to_string())
}

#[cfg(not(feature = "hydrate"))]
pub async fn check_wallet_available() -> bool {
    false
}
