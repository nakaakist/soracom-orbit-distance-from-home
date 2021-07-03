# Rust development environment for SORACOM Orbit

## Development

See developer's guide for detail. Don't forget to click `Reopen in Container` when you launch the workspace going forward.

1. Open and edit [`src/lib.rs`](src/lib.rs) which is your Rust for SORACOM Orbit.
2. Run `build` script to build WASM module under `target/` directory. WASM module will be built as `target/wasm32-unknown-unknown/debug/soralet.wasm`.
   - Open integrated terminal and run `cargo build`, or
   - Trigger **Run Build Task** from the VS Code **Command Palette** (<kbd>⇧⌘P</kbd>), or
   - <kbd>⇧⌘B</kbd>

Please note that you have to export `uplink()` and/or `downlink()` with `#[no_mangle]` macro in your Rust code.

```rust
// For processing uplink (UE to SORACOM)
#[no_mangle]
pub fn uplink() -> i32 {
    /* code */
}

// For processing downlink (SORACOM to UE)
pub fn downlink() -> i32 {
  /* code */
}
```

## Testing

No test at this moment.

## Deployment

1. Run `cargo build --release` to build optimized WASM module under `target/` directory. WASM module will be built as `target/wasm32-unknown-unknown/release/soralet.wasm`.
2. See developer's guide.
