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
use commit_verify::{commit_encode, CommitVerify, ConsensusCommit, PrehashedProtocol, TaggedHash};

use crate::ContainerId;

// "storm:message"
static MIDSTATE_MESG_ID: [u8; 32] = [
    12, 61, 136, 60, 191, 129, 135, 229, 141, 35, 41, 161, 203, 125, 0, 101, 109, 136, 50, 236, 7,
    101, 59, 39, 148, 207, 63, 236, 255, 48, 24, 171,
];

/// Tag used for [`MesgId`] hash type
pub struct MesgIdTag;

impl sha256t::Tag for MesgIdTag {
    #[inline]
    fn engine() -> sha256::HashEngine {
        let midstate = sha256::Midstate::from_inner(MIDSTATE_MESG_ID);
        sha256::HashEngine::from_midstate(midstate, 64)
    }
}

/// Unique messag identifier
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
#[derive(Wrapper, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
#[derive(StrictEncode, StrictDecode)]
#[wrapper(Debug, Display)]
pub struct MesgId(sha256t::Hash<MesgIdTag>);

impl<Msg> CommitVerify<Msg, PrehashedProtocol> for MesgId
where Msg: AsRef<[u8]>
{
    #[inline]
    fn commit(msg: &Msg) -> MesgId { MesgId::hash(msg) }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Display, AsAny)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
#[display("Mesg")]
pub struct Mesg {
    pub parent_id: MesgId,
    pub data: Vec<u8>,
    pub container_id: Option<ContainerId>,
}

impl commit_encode::Strategy for Mesg {
    type Strategy = commit_encode::strategies::UsingStrict;
}

impl ConsensusCommit for Mesg {
    type Commitment = MesgId;
}

impl Mesg {
    pub fn mesg_id(&self) -> MesgId { self.consensus_commit() }
}

#[cfg(test)]
mod test {
    use amplify::Wrapper;
    use commit_verify::tagged_hash;

    use super::*;

    #[test]
    fn test_container_id_midstate() {
        let midstate = tagged_hash::Midstate::with(b"storm:message");
        assert_eq!(midstate.into_inner().into_inner(), MIDSTATE_MESG_ID);
    }
}
