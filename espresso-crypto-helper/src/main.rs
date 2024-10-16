#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    espresso_crypto_helper::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}

#[test]
fn test_data() {
    use espresso_crypto_helper::verify_namespace_helper;
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Serialize, Deserialize)]
    struct TestData {
        ns_proof: Vec<u8>,
        vid_commit: Vec<u8>,
        vid_common: Vec<u8>,
        namespace: u64,
        tx_commit: Vec<u8>,
        ns_table: Vec<u8>,
    }

    let data = fs::read_to_string("./test_data.json").unwrap();
    let data: TestData = serde_json::from_str(&data).unwrap();
    let result = verify_namespace_helper(
        data.namespace,
        &data.ns_proof,
        &data.vid_commit,
        &data.ns_table,
        &data.tx_commit,
        &data.vid_common,
    );
    assert!(result)
}
