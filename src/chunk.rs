// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::ops::{Deref, DerefMut};

use bitcoin_hashes::sha256;
use commit_verify::{commit_encode, ConsensusCommit};
use strict_encoding::MediumVec;

pub type ChunkId = sha256::Hash;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("<chunk>")]
pub struct Chunk(MediumVec<u8>);

impl Deref for Chunk {
    type Target = MediumVec<u8>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Chunk {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] { self.0.as_ref() }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = strict_encoding::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        MediumVec::try_from(slice.to_vec()).map(Self)
    }
}

impl commit_encode::Strategy for Chunk {
    type Strategy = commit_encode::strategies::UsingStrict;
}

impl ConsensusCommit for Chunk {
    type Commitment = ChunkId;
}

impl Chunk {
    pub fn chunk_id(&self) -> ChunkId { self.consensus_commit() }
}
