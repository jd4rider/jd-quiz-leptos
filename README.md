To set up your Rust toolchain using `nightly` (and add the ability to compile Rust to WebAssembly, if you haven’t already)

```
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```

To install trunk

```
# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```

To run the app execute the following:

```lang-none
trunk build
cd server
npm install
npm run start:nodemon
```

To make changes to the tailwind.css file go to https://github.com/matiu2/tailwind-yew-builder

To learn how to use Leptos visit https://github.com/leptos-rs/leptos
