#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

mod bytes;
mod full_payload;
mod hotshot_types;
mod namespace_payload;
mod sequencer_data_structures;
mod uint_bytes;
mod utils;
mod v0_3;

use std::num::NonZeroU32;

use ark_ff::PrimeField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use committable::{Commitment, Committable};
use ethers_core::types::U256;
use full_payload::{NsProof, NsTable};
use getrandom::{register_custom_getrandom, Error};
use hotshot_types::{VidCommitment, VidCommon};
use jf_crhf::CRHF;
use jf_merkle_tree::prelude::{
    MerkleCommitment, MerkleNode, MerkleProof, MerkleTreeScheme, Sha3Node,
};
use jf_rescue::{crhf::VariableLengthRescueCRHF, RescueError};
use sequencer_data_structures::{
    field_to_u256, BlockMerkleCommitment, BlockMerkleTree, Header, Transaction,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tagged_base64::TaggedBase64;

use stylus_sdk::prelude::*;

#[storage]
#[entrypoint]
pub struct Entry {}

#[public]
impl Entry {}
