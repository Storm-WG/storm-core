// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use bitcoin_hashes::{sha256, sha256t};
use commit_verify::{
    commit_encode, CommitVerify, ConsensusCommit, PrehashedProtocol, TaggedHash,
};
use stens::AsciiString;
use strict_encoding::{MediumVec, StrictEncode};

use crate::{ChunkId, MesgId};

// "storm:container"
static MIDSTATE_CONTAINER_ID: [u8; 32] = [
    12, 61, 136, 60, 191, 129, 135, 229, 141, 35, 41, 161, 203, 125, 0, 101,
    109, 136, 50, 236, 7, 101, 59, 39, 148, 207, 63, 236, 255, 48, 24, 171,
];

/// Tag used for [`ContainerId`] hash type
pub struct ContainerIdTag;

impl sha256t::Tag for ContainerIdTag {
    #[inline]
    fn engine() -> sha256::HashEngine {
        let midstate = sha256::Midstate::from_inner(MIDSTATE_CONTAINER_ID);
        sha256::HashEngine::from_midstate(midstate, 64)
    }
}

/// Unique data container identifier
#[derive(
    Wrapper, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From
)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
#[wrapper(Debug, Display)]
pub struct ContainerId(sha256t::Hash<ContainerIdTag>);

impl<Msg> CommitVerify<Msg, PrehashedProtocol> for ContainerId
where Msg: AsRef<[u8]>
{
    #[inline]
    fn commit(msg: &Msg) -> ContainerId { ContainerId::hash(msg) }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[display("{container_id}@{message_id}")]
pub struct ContainerFullId {
    /// Message defining access rights to the container.
    pub message_id: MesgId,
    pub container_id: ContainerId,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, AsAny)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Container {
    /// Version of the container. Always 0 for now.
    pub version: u16,
    /// MIME type of the file.
    pub mime: AsciiString,
    /// UTF-8 description of the file.
    pub info: String,
    /// Container size, which is the sum of sizes of the individual chunks.
    ///
    /// Consensus limitation of the container size is 43 bits: 19 bits for the
    /// number of chunks and up to 24 bits for chunk size. 19 bits for the max
    /// number of chunks comes from the fact that the total size of the
    /// container index must be below 2^24 bytes (to fit into a LN packet);
    /// since the size of the chunk id is 2^5 (32) bits, and the maximum
    /// Bifrost packet size is 2^24, we have only 24-5=19 bits to store the
    /// chunk index.
    pub size: u64,
    pub chunks: MediumVec<ChunkId>,
}

impl commit_encode::Strategy for Container {
    type Strategy = commit_encode::strategies::UsingStrict;
}

impl ConsensusCommit for Container {
    type Commitment = ContainerId;
}

impl Container {
    pub fn container_id(&self) -> ContainerId { self.consensus_commit() }
}

#[cfg(test)]
mod test {
    use amplify::Wrapper;
    use commit_verify::tagged_hash;

    use super::*;

    #[test]
    fn test_container_id_midstate() {
        let midstate = tagged_hash::Midstate::with(b"storm:container");
        assert_eq!(midstate.into_inner().into_inner(), MIDSTATE_CONTAINER_ID);
    }
}
