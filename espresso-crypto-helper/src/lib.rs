#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

pub mod bytes;
pub mod full_payload;
pub mod hotshot_types;
pub mod namespace_payload;
pub mod sequencer_data_structures;
pub mod uint_bytes;
pub mod utils;
pub mod v0_3;

use std::num::NonZeroU32;

use ark_ff::PrimeField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use committable::{Commitment, Committable};
use ethers_core::types::U256;
use full_payload::{NsProof, NsTable};
use getrandom::{register_custom_getrandom, Error};
use hotshot_types::{VidCommitment, VidCommon};
use jf_crhf::CRHF;
use jf_merkle_tree::{
    hasher::{HasherDigest, HasherNode},
    prelude::{MerkleNode, MerkleProof, MerkleTreeScheme, Sha3Node},
    MerkleCommitment,
};
use jf_rescue::{crhf::VariableLengthRescueCRHF, RescueError};
use sequencer_data_structures::{field_to_u256, BlockMerkleTree, Header, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tagged_base64::TaggedBase64;

use stylus_sdk::prelude::*;
use v0_3::BlockMerkleCommitment;

const MY_CUSTOM_ERROR_CODE: u32 = Error::CUSTOM_START + 42;
pub fn always_fail(_buf: &mut [u8]) -> Result<(), Error> {
    let code = NonZeroU32::new(MY_CUSTOM_ERROR_CODE).unwrap();
    Err(Error::from(code))
}

register_custom_getrandom!(always_fail);

#[storage]
#[entrypoint]
pub struct Entry {}

#[public]
impl Entry {
    // pub fn verify_namespace(
    //     namespace: u64,
    //     proof_bytes: Vec<u8>,
    //     commit_bytes: Vec<u8>,
    //     ns_table_bytes: Vec<u8>,
    //     tx_comm_bytes: Vec<u8>,
    //     common_data_bytes: Vec<u8>,
    // ) -> bool {
    //     return verify_namespace_helper(
    //         namespace,
    //         &proof_bytes,
    //         &commit_bytes,
    //         &ns_table_bytes,
    //         &tx_comm_bytes,
    //         &common_data_bytes,
    //     );
    // }

    pub fn verify_merkle_proof(
        proof_bytes: Vec<u8>,
        block_comm_bytes: Vec<u8>,
        circuit_block_bytes: Vec<u8>,
        height: u64,
        header_commitment: Vec<u8>,
    ) -> bool {
        return verify_merkle_proof_helper(
            &proof_bytes,
            &block_comm_bytes,
            &circuit_block_bytes,
            height,
            &header_commitment,
        );
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    CanonicalDeserialize,
    CanonicalSerialize,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct NamespaceId(u64);

impl From<NamespaceId> for u32 {
    fn from(value: NamespaceId) -> Self {
        value.0 as Self
    }
}

impl From<u32> for NamespaceId {
    fn from(value: u32) -> Self {
        Self(value as u64)
    }
}

// pub type VidScheme = Advz<Bn254, sha2::Sha256>;
pub type Proof = Vec<MerkleNode<Commitment<Header>, u64, Sha3Node>>;
pub type CircuitField = ark_ed_on_bn254::Fq;

// Helper function to verify a block merkle proof.
pub fn verify_merkle_proof_helper(
    proof_bytes: &[u8],
    block_comm: &[u8], // [u8; 32]
    hotshot_commitment: &[u8],
    height: u64,
    header_commitment: &[u8], // [u8; 32]
) -> bool {
    let proof: Proof = serde_json::from_slice(proof_bytes).unwrap();

    let proof = MerkleProof::new(height, proof.to_vec());

    let proved_comm: &[u8] = proof.elem().unwrap().as_ref();

    if proved_comm != header_commitment {
        return false;
    }

    // let block_comm: [u8; 32] = block_comm.try_into().unwrap();
    // let block_comm: Sha3Node = unsafe { std::mem::transmute(block_comm) };
    let block_comm_str = std::str::from_utf8(block_comm).unwrap();
    let tagged = TaggedBase64::parse(&block_comm_str).unwrap();
    let block_comm: BlockMerkleCommitment = tagged.try_into().unwrap();
    BlockMerkleTree::verify(block_comm.digest(), height, proof)
        .unwrap()
        .unwrap();

    let mut block_comm_root_bytes = vec![];
    block_comm
        .serialize_compressed(&mut block_comm_root_bytes)
        .unwrap();
    let field_bytes = hash_bytes_to_field(&block_comm_root_bytes).unwrap();
    let local_block_comm_u256 = field_to_u256(field_bytes);
    let circuit_block_comm_u256 = U256::from_little_endian(hotshot_commitment);

    return local_block_comm_u256 == circuit_block_comm_u256;
}

// Helper function to verify a VID namespace proof that takes the byte representations of the proof,
// namespace table, and commitment string.
pub fn verify_namespace_helper(
    namespace: u64,
    proof_bytes: &[u8],
    commit_bytes: &[u8],
    ns_table_bytes: &[u8],
    tx_comm_bytes: &[u8],
    common_data_bytes: &[u8],
) -> bool {
    let commit_str = std::str::from_utf8(commit_bytes).unwrap();
    let txn_comm_str = std::str::from_utf8(tx_comm_bytes).unwrap();

    let proof: NsProof = serde_json::from_slice(proof_bytes).unwrap();
    let ns_table: NsTable = NsTable {
        bytes: ns_table_bytes.to_vec(),
    };
    let tagged = TaggedBase64::parse(&commit_str).unwrap();
    let commit: VidCommitment = tagged.try_into().unwrap();
    let vid_common: VidCommon = serde_json::from_slice(common_data_bytes).unwrap();

    let (txns, ns) = proof.verify(&ns_table, &commit, &vid_common).unwrap();

    let namespace: u32 = namespace.try_into().unwrap();
    let txns_comm = hash_txns(namespace, &txns);

    return ns == namespace.into() && txns_comm == txn_comm_str;
}

// TODO: Use Commit trait: https://github.com/EspressoSystems/nitro-espresso-integration/issues/88
fn hash_txns(namespace: u32, txns: &[Transaction]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(namespace.to_le_bytes());
    for txn in txns {
        hasher.update(&txn.payload);
    }
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
}

fn hash_bytes_to_field(bytes: &[u8]) -> Result<CircuitField, RescueError> {
    // make sure that `mod_order` won't happen.
    let bytes_len = ((<CircuitField as PrimeField>::MODULUS_BIT_SIZE + 7) / 8 - 1) as usize;
    let elem = bytes
        .chunks(bytes_len)
        .map(CircuitField::from_le_bytes_mod_order)
        .collect::<Vec<_>>();
    Ok(VariableLengthRescueCRHF::<_, 1>::evaluate(elem)?[0])
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use committable::Committable;
    use serde::Deserialize;
    use tagged_base64::TaggedBase64;

    use crate::{
        sequencer_data_structures::Header, v0_3::BlockMerkleCommitment, verify_merkle_proof_helper,
        Proof,
    };

    #[test]
    fn check_data() {
        let s = "MERKLE_COMM~hgo2QfXdj-YpouFIHiVJQtU2Gtgrwjr3i1F_kmspKH8gAAAAAAAAACwAAAAAAAAArg";
        let a = TaggedBase64::from_str(s).unwrap();
        let b: BlockMerkleCommitment = a.try_into().unwrap();
        println!("{:?}", b);
    }

    #[test]
    pub fn test_merkle_proof_verification() {
        let s = include_str!("../merkle_proof_test_data.json");
        let test_data: TestData = serde_json::de::from_str(s).unwrap();
        let proof = &test_data.proof;
        let proof = serde_json::to_vec(proof).unwrap();
        let header_commit = test_data.header.commit();
        let bytes: &[u8] = header_commit.as_ref();
        println!("{:?}", header_commit);
        // let block_comm: BlockMerkleCommitment = test_data.block_merkle_root.try_into().unwrap();
        // println!("block: {:?}", block_comm);
        println!("height: {:?}", test_data.header.height());
        let result = verify_merkle_proof_helper(
            &proof,
            // &[
            //     134, 10, 54, 65, 245, 221, 143, 230, 41, 162, 225, 72, 30, 37, 73, 66, 213, 54, 26,
            //     216, 43, 194, 58, 247, 139, 81, 127, 146, 107, 41, 40, 127,
            // ],
            test_data.block_merkle_root.to_string().as_bytes(),
            &test_data.hotshot_commitment,
            test_data.header.height(),
            bytes,
        );
        assert!(result)
    }

    #[derive(Deserialize)]
    struct TestData {
        proof: Proof,
        header: Header,
        block_merkle_root: TaggedBase64,
        // header_string: String,
        hotshot_commitment: Vec<u8>,
    }
}
