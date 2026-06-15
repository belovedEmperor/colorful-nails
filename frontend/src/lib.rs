#![recursion_limit = "512"]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[expect(unused)]
    // required for wasm_bindgen
    use app::App;
    #[expect(unused)]
    // has no server function so must be explicitly anchored
    use app::Header;

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_islands();
}
