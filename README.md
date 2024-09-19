### Installation

- Option 1, nix: Either `nix develop` or `direnv allow`
- Option 2, rustup: `rustup target add wasm32-unknown-unknown`

Install cargo-stylus

    cargo install cargo-stylus

Build stylus project

    cd foo
    cargo build --release --target wasm32-unknown-unknown
    cargo stylus check --verbose --wasm-file target/wasm32-unknown-unknown/release/stylus_hello_world.wasm

