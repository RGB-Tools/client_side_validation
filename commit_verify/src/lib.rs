// LNP/BP client-side-validation library implementing respective LNPBP
// specifications & standards (LNPBP-7, 8, 9, 42)
//
// Written in 2019-2021 by
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

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[macro_use]
extern crate bitcoin_hashes;
#[cfg(feature = "serde")]
extern crate serde_crate as serde;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_with;

pub mod api;
pub mod commit_encode;
mod digests;
pub mod multi_commit;
mod slice32;
pub mod tagged_hash;

#[doc(hidden)]
pub use api::{CommitVerify, EmbedCommitVerify, TryCommitVerify};
#[doc(hidden)]
pub use commit_encode::{
    merklize, CommitConceal, CommitEncode, ConsensusCommit,
    ConsensusMerkleCommit, MerkleSource, ToMerkleSource,
};
#[doc(hidden)]
pub use multi_commit::{Message, MultiCommitBlock, MultiCommitItem};
pub use slice32::Slice32;
#[doc(hidden)]
pub use tagged_hash::TaggedHash;
