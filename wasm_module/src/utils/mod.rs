// https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/template-deep-dive/src-utils-rs.html
// https://github.com/rustwasm/console_error_panic_hook
pub fn set_panic_hook() {
    // Tells Rust to check if the console_error_panic_hook feature is set at compile time. If it is, it will call this function.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
