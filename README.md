# pdbtbx-wit

Wrapper around [pdbtbx rust library](https://crates.io/crates/pdbtbx) for reading (crystallographic) Protein Data Bank (PDB) and mmCIF files in JavaScript.

Using [WebAssembly Component and Wasm Interface Type](https://component-model.bytecodealliance.org/language-support/rust.html)

## Usage

```javascript

```shell
# Install dependencies
cargo install cargo-component
npm install

# Regen src/bindings.rs + build wasm
cargo component build --release

# Display wit from wasm
npx jco wit target/wasm32-wasi/release/pdbtbx_wit.wasm

# Create js wrapper around wasm
npm run build

# Exercize
node test.js
```