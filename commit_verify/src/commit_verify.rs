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

//! Base commit-verify scheme interface.

/// Trait for commit-verify scheme. A message for the commitment may be any
/// structure that can be represented as a byte array (i.e. implements
/// `AsRef<[u8]>`).
pub trait CommitVerify<Msg>
where
    Self: Eq + Sized,
{
    /// Creates a commitment to a byte representation of a given message
    fn commit(msg: &Msg) -> Self;

    /// Verifies commitment against the message; default implementation just
    /// repeats the commitment to the message and check it against the `self`.
    #[inline]
    fn verify(&self, msg: &Msg) -> bool { Self::commit(msg) == *self }
}

/// Trait for a failable version of commit-verify scheme. A message for the
/// commitment may be any structure that can be represented as a byte array
/// (i.e. implements `AsRef<[u8]>`).
pub trait TryCommitVerify<Msg>
where
    Self: Eq + Sized,
{
    /// Error type that may be reported during [`TryCommitVerify::try_commit`]
    /// and [`TryCommitVerify::try_verify`] procedures
    type Error: std::error::Error;

    /// Tries to create commitment to a byte representation of a given message
    fn try_commit(msg: &Msg) -> Result<Self, Self::Error>;

    /// Tries to verify commitment against the message; default implementation
    /// just repeats the commitment to the message and check it against the
    /// `self`.
    #[inline]
    fn try_verify(&self, msg: &Msg) -> Result<bool, Self::Error> {
        Ok(Self::try_commit(msg)? == *self)
    }
}

/// Helpers for writing test functions working with commit-verify scheme
#[cfg(test)]
pub mod test_helpers {
    use core::fmt::Debug;
    use core::hash::Hash;
    use std::collections::HashSet;

    use bitcoin_hashes::hex::FromHex;

    use super::*;

    /// Generates a set of messages for testing purposes
    ///
    /// All of these messages MUST produce different commitments, otherwise the
    /// commitment algorithm is not collision-resistant
    pub fn gen_messages() -> Vec<Vec<u8>> {
        vec![
            // empty message
            b"".to_vec(),
            // zero byte message
            b"\x00".to_vec(),
            // text message
            b"test".to_vec(),
            // text length-extended message
            b"test*".to_vec(),
            // short binary message
            Vec::from_hex("deadbeef").unwrap(),
            // length-extended version
            Vec::from_hex("deadbeef00").unwrap(),
            // prefixed version
            Vec::from_hex("00deadbeef").unwrap(),
            // serialized public key as text
            b"0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".to_vec(),
            // the same public key binary data
            Vec::from_hex("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                .unwrap(),
            // different public key
            Vec::from_hex("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9")
                .unwrap(),
        ]
    }

    /// Runs round-trip of commitment and verification for a given set of
    /// messages
    pub fn commit_verify_suite<MSG, CMT>(messages: Vec<MSG>)
    where
        MSG: AsRef<[u8]> + Eq,
        CMT: CommitVerify<MSG> + Eq + Hash + Debug,
    {
        messages.iter().fold(
            HashSet::<CMT>::with_capacity(messages.len()),
            |mut acc, msg| {
                let commitment = CMT::commit(msg);

                // Commitments MUST be deterministic: each message should
                // produce unique commitment
                (1..10).for_each(|_| {
                    assert_eq!(CMT::commit(msg), commitment);
                });

                // Testing verification
                assert!(commitment.verify(msg));

                messages.iter().for_each(|m| {
                    // Testing that commitment verification succeeds only
                    // for the original message and fails for the rest
                    assert_eq!(commitment.verify(m), m == msg);
                });

                acc.iter().for_each(|cmt| {
                    // Testing that verification against other commitments
                    // returns `false`
                    assert!(!cmt.verify(msg));
                });

                // Detecting collision
                assert!(acc.insert(commitment));

                acc
            },
        );
    }
}

#[cfg(test)]
mod test {
    use core::fmt::Debug;
    use core::hash::Hash;

    use bitcoin_hashes::sha256d;

    use super::test_helpers::*;
    use super::*;

    #[derive(Debug, Display, Error)]
    #[display(Debug)]
    struct Error;
    #[derive(Clone, PartialEq, Eq, Debug, Hash)]
    struct DummyHashCommitment(sha256d::Hash);
    impl<T> CommitVerify<T> for DummyHashCommitment
    where
        T: AsRef<[u8]>,
    {
        fn commit(msg: &T) -> Self {
            Self(bitcoin_hashes::Hash::hash(msg.as_ref()))
        }
    }

    #[test]
    fn test_commit_verify() {
        commit_verify_suite::<Vec<u8>, DummyHashCommitment>(gen_messages());
    }
}
