# Outer-noise

```sh
cargo build --target=wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/outer_noise.wasm 

# Generates img.png testing the noise generator on a large area
deno -A test.js
```