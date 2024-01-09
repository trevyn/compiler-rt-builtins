# compiler-rt-builtins

The LLVM compiler-rt builtins source, conveniently packaged as a Rust crate.

```toml
[dependencies]
compiler-rt-builtins = "0.1"
```

In your `build.rs`:

```rust
println!("cargo:rustc-link-lib=compiler-rt-builtins");
```

Created because when compiling C code to `wasm32-unknown-unknown`, some builtins are missing from Rust's `compiler_builtins`. Might have other uses too, who knows?
