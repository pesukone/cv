pub mod app;

leptos_i18n::load_locales!();

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  use crate::app::*;
  console_error_panic_hook::set_once();
  leptos::mount::hydrate_islands();
}
