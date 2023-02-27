// LNP/BP client-side-validation foundation libraries implementing LNPBP
// specifications & standards (LNPBP-4, 7, 8, 9, 42, 81)
//
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the Apache 2.0 License along with this
// software. If not, see <https://opensource.org/licenses/Apache-2.0>.

// Coding conventions
#![recursion_limit = "256"]
#![deny(dead_code, missing_docs, warnings)]

//! Library providing primitives for cryptographic commit-verify schemes used in
//! client-side-validation
//!
//! Library covers [LNPBP-9] and [LNPBP-81] standards.
//!
//! [LNPBP-9]: https://github.com/LNP-BP/LNPBPs/blob/master/lnpbp-0009.md
//! [LNPBP-81]: https://github.com/LNP-BP/LNPBPs/blob/master/lnpbp-0081.md

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate bitcoin_hashes;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;
#[cfg(feature = "lnpbp_secp256k1zkp")]
extern crate lnpbp_secp256k1zkp as secp256k1zkp;
#[cfg(feature = "serde")]
extern crate serde_with;

pub mod commit_encode;
pub mod commit_verify;
pub mod convolve_commit;
pub mod embed_commit;
pub mod lnpbp4;
pub mod merkle;
pub mod tagged_hash;

pub use commit_encode::{CommitConceal, CommitEncode, ConsensusCommit};
pub use embed_commit::{
    EmbedCommitProof, EmbedCommitProofStatic, EmbedCommitVerify,
    EmbedCommitVerifyStatic,
};
pub use merkle::{
    merklize, ConsensusMerkleCommit, MerkleSource, ToMerkleSource,
};
pub use tagged_hash::TaggedHash;

pub use crate::commit_verify::{
    CommitVerify, TryCommitVerify, TryCommitVerifyStatic,
};

// TODO: Improve support of creating tagged hashes of the messages at the
//       commitment protocol level.

/// Marker trait for specific commitment protocols.
///
/// Generic parameter `Protocol` used in commitment scheme traits provides a
/// context & configuration for the concrete implementations.
///
/// Introduction of such generic allows to:
/// - implement trait for foreign data types;
/// - add multiple implementations under different commitment protocols to the
///   combination of the same message and container type (each of each will have
///   its own `Proof` type defined as an associated generic).
///
/// Each of the commitment protocols should use [`Self::HASH_TAG_MIDSTATE`] as a
/// part of tagged hashing of the message as a part of the commitment procedure.
pub trait CommitmentProtocol {
    /// Midstate for the protocol-specific tagged hash.
    const HASH_TAG_MIDSTATE: Option<bitcoin_hashes::sha256::Midstate>;
}

/// Protocol defining commits created by using externally created hash value
/// *optionally pretagged).
pub struct PrehashedProtocol;
impl CommitmentProtocol for PrehashedProtocol {
    const HASH_TAG_MIDSTATE: Option<bitcoin_hashes::sha256::Midstate> = None;
}
