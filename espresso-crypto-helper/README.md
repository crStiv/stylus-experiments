# Espresso Crypto Helper

## Nix develop

```cmd
nix develop
```

## Build the contract

```cmd
cargo build --release --target wasm32-unknown-unknown
```

## Run the test node

Currently we need to run a testnode to get around the size limitations.

Use a new terminal and in your `nitro-espresso-integration` repo:

```cmd
git pull
git checkout jh/stylus-experiment
git submodule update --remote nitro-testnode
cd nitro-testnode
./test-node.bash --init-force --dev
```

## Concat the contract WASM with soft-float implementation

### Create an output directory

```cmd
mkdir tmp
```

### Run the python script

Replace the `{YOUR_INTEGRATION_REPO_PATH}` with your path.

```cmd
python3 ./build.py ./target/wasm32-unknown-unknown/release/espresso_crypto_helper.wasm {YOUR_INTEGRATION_REPO_PATH}/target/machines/latest/soft-float.wasm ./tmp
```

After this command, you will have these stuff in your directory:

- espresso_crypto_helper.wasm
- espresso_crypto_helper.wat
- soft-float.wat

What we only need is the WASM file.

### Run the stylus check

```cmd
cargo stylus check --wasm-file ./tmp/espresso-crypto-helper.wasm -e http://localhost:8547
```

### Deploy the contract

```cmd
cargo stylus deploy --verbose --wasm-file target/wasm32-unknown-unknown/release/stylus_hello_world.wasm --endpoint http://localhost:8547 --private-key 0xdc04c5399f82306ec4b4d654a342f40e2e0620fe39950d967e1e574b32d4dd36
```
