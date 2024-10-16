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

### Generate the abi

Check if the ABI can be generated correctly

```cmd
cargo stylus export-abi
```

### Call the contract

`test_data.json` has the data that we can build a calldata to call the contract. These data are from [a test in espresso-sequencer](https://github.com/EspressoSystems/espresso-sequencer/blob/f0ec645cb27e224f98bf490147cefeca7bd62882/types/src/v0/impls/block/full_payload/ns_proof/test.rs#L79).

```cmd
python3 call_contract.py
```

Then you can see the receipt like this:

```output
AttributeDict({'blockHash': HexBytes('0x6b9e69fa41eb66cb715507e6b85892bad6a3670e5d45d60f854856f20f35816f'), 'blockNumber': 27, 'contractAddress': None, 'cumulativeGasUsed': 138917, 'effectiveGasPrice': 100000000, 'from': '0x5E1497dD1f08C87b2d8FE23e9AAB6c1De833D927', 'gasUsed': 138917, 'gasUsedForL1': '0x0', 'l1BlockNumber': '0x9e9', 'logs': [], 'logsBloom': HexBytes('0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'), 'status': 0, 'to': '0x4e5b65FB12d4165E22f5861D97A33BA45c006114', 'transactionHash': HexBytes('0x70fc3812deed01c37d254857ba1ffe12e7968ca1be3bb04dcb7379548fda1302'), 'transactionIndex': 1, 'type': 0})
```
