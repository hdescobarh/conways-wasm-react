# conways-wasm &#129408; &#128376;

![Rust](https://img.shields.io/badge/-Rust-B7410E?logo=rust&logoColor=28282B&labelColor=white)
![WebAssembly](https://img.shields.io/badge/-WebAssembly-393939?logo=webassembly&logoColor=654FF0&labelColor=white)
![License](https://img.shields.io/badge/license-MIT-blue)

## Description

A simple Conway's Game of Life [0] exercises intended to introduce myself to Rust WebAssembly. The simulation space is **toroidal** and implements a basic memoization strategy.

The excersice and initial project's setting was inspired by [1-3].

## Regarding Panic and file size

Setting "panic=abort" [4] does not remplace "process:abort" [5]. In consecuence, a panic that aborts still calls some functions (e.g., panic_fmt, set_hook) [6,7]; then, it will put code bload on the wasm.

## References

0. [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
1. [wasm-bindgen-test's Guide](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html)
2. [wasm-pack-template] https://github.com/rustwasm/wasm-pack-template
3. [wasm_game_of_life](https://github.com/rustwasm/wasm_game_of_life)
4. [Build configuration: Abort on panic! (The Rust Performance Book)](https://nnethercote.github.io/perf-book/build-configuration.html#abort-on-panic)
5. [Shrinking .wasm Code Size: Avoid Panicking (Rust and WebAssembly)](https://rustwasm.github.io/docs/book/reference/code-size.html#avoid-panicking)
6. [Avoiding allocations in Rust to shrink Wasm modules](https://www.reddit.com/r/rust/comments/xxvwyy/avoiding_allocations_in_rust_to_shrink_wasm/)
7. [Function std::process::abort](https://doc.rust-lang.org/std/process/fn.abort.html)
