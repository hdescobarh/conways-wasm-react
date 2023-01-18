# conways-wasm &#129408;

![Rust](https://img.shields.io/badge/-Rust-B7410E?logo=rust&logoColor=28282B&labelColor=white)
![WebAssembly](https://img.shields.io/badge/-WebAssembly-393939?logo=webassembly&logoColor=654FF0&labelColor=white)
![License](https://img.shields.io/badge/license-MIT-blue)

## wasm file size: "panic = abort" vs. std::process::abort

Setting "panic=abort" (1) does not remplace "process:abort" (2). In consecuence, a panic that aborts still calls some functions (e.g., panic_fmt, set_hook) (3, 4); then, it will put code bload on the wasm.

- (1) [Build configuration: Abort on panic!](https://nnethercote.github.io/perf-book/build-configuration.html#abort-on-panic)
- (2) [Shrinking .wasm Code Size: Avoid Panicking (Rust and WebAssembly)](https://rustwasm.github.io/docs/book/reference/code-size.html#avoid-panicking)
- (4) [Avoiding allocations in Rust to shrink Wasm modules](https://www.reddit.com/r/rust/comments/xxvwyy/avoiding_allocations_in_rust_to_shrink_wasm/)
- (3) [Function std::process::abort](https://doc.rust-lang.org/std/process/fn.abort.html)

- () [General Tips (The Rust Performance Book)](https://nnethercote.github.io/perf-book/general-tips.html)

# References used for project setting

The initial template is inspired by the examples in:

- [wasm-bindgen-test's Guide](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html)
- [wasm-pack-template] https://github.com/rustwasm/wasm-pack-template
- [wasm_game_of_life](https://github.com/rustwasm/wasm_game_of_life)
