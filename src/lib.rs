pub mod app;
pub mod error;
pub mod server_fns;

pub use error::{env_or_error, Error, Result};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
