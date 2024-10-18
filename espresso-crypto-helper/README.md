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

Currently we need to run our dev node to get around the size limitations.

Open a new terminal and run these commands in your `nitro-espresso-integration` repo and checkout to `jh/stylus-experiment` branch:

```cmd
git pull
git checkout jh/stylus-experiment
git submodule update --remote nitro-testnode
git submodule update --remote go-ethereum
cd nitro-testnode
./test-node.bash --init-force --dev
```

## Concat the contract WASM with soft-float implementation

`f64` instructions couldn't be converted into WAVM easily because solidity has no `f64` type. Unfornately our contract WASM somehow has some f64 instructions, which hinders it being deployed.

`Nitro` has similar issues and they use the `soft-float` implementation and the cross-module call amoung the WASMs to turn the `replay binary WASM` to `WAVM`.
But `Nitro` doesn't support the `f64` conversions in the stylus contract executing environment.
We have to hack our WASM to get around this.

### Prepare the soft-float WASM

The `soft-float` WASM is under the current directory.

You can also build it in your `nitro-espresso-integration` repo with this command :

```cmd
make build-wasm-libs
```

Then the `soft-float.wasm` can be found in `./target/machines/latest/soft-float.wasm`.

### Create an output directory

```cmd
mkdir tmp
```

### Run the python script

```cmd
python3 ./build.py ./target/wasm32-unknown-unknown/release/espresso_crypto_helper.wasm ./soft-float.wasm ./tmp
```

After this command, you will have these stuff in your directory:

- espresso_crypto_helper.wasm
- espresso_crypto_helper.wat
- soft-float.wat

This script converts those 2 WASM into the WAT format, and appends the `soft-float` implementation to our contract file. And then it turns the contract WAT file into WASM again.
What we only need is the WASM file.

### Run the stylus check

```cmd
cargo stylus check --wasm-file ./tmp/espresso_crypto_helper.wasm -e http://localhost:8547
```

## Deploy the contract

```cmd
cargo stylus deploy --wasm-file ./tmp/espresso_crypto_helper.wasm --private-key 0xdc04c5399f82306ec4b4d654a342f40e2e0620fe39950d967e1e574b32d4dd36 --endpoint http://localhost:8547 --no-verify
```

## Generate the abi

Check if the ABI can be generated correctly

```cmd
cargo stylus export-abi
```

## Call the contract

```cmd
python3 call_contract.py
```

This script is using the `test_data.json` as the input to call the contract.

`test_data.json` has the data that we can build a calldata to call the contract. These data are from [a test in espresso-sequencer](https://github.com/EspressoSystems/espresso-sequencer/blob/f0ec645cb27e224f98bf490147cefeca7bd62882/types/src/v0/impls/block/full_payload/ns_proof/test.rs#L79) with the `num_of_storage_nodes` in the vid scheme set to 4.

Then you should see the output like this:

```output
0x0000000000000000000000000000000000000000000000000000000000000001
```
