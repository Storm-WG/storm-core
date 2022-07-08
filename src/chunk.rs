// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::convert::Infallible;
use std::ops::{Deref, DerefMut};

use bitcoin_hashes::{sha256, Hash};
use commit_verify::{commit_encode, ConsensusCommit};
use strict_encoding::{MediumVec, StrictEncode};

use crate::ContainerId;

/// ChunkId is a non-tagged hash of all of the chunk data. It is a single hash
/// such that it can be length-extended; i.e. chunks are composable.
pub type ChunkId = sha256::Hash;

pub trait ChunkIdExt {
    fn with_fragments(
        a: impl StrictEncode,
        b: impl StrictEncode,
    ) -> Result<ChunkId, strict_encoding::Error> {
        let mut engine = ChunkId::engine();
        a.strict_encode(&mut engine)?;
        b.strict_encode(&mut engine)?;
        Ok(ChunkId::from_engine(engine))
    }

    fn with_fixed_fragments(
        a: impl StrictEncode,
        b: impl StrictEncode,
    ) -> ChunkId {
        let mut engine = ChunkId::engine();
        a.strict_encode(&mut engine)
            .expect("chunk data must be strict-encodable");
        b.strict_encode(&mut engine)
            .expect("chunk data must be strict-encodable");
        ChunkId::from_engine(engine)
    }

    fn try_from(
        data: impl StrictEncode,
    ) -> Result<ChunkId, strict_encoding::Error> {
        let mut engine = ChunkId::engine();
        data.strict_encode(&mut engine)?;
        Ok(ChunkId::from_engine(engine))
    }
}

impl ChunkIdExt for ChunkId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display)]
#[derive(StrictEncode, StrictDecode)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[display("{chunk_id}@{container_id}")]
pub struct ChunkFullId {
    pub container_id: ContainerId,
    pub chunk_id: ChunkId,
}

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display, Error
)]
#[display("data too large to produce a single chunk")]
pub struct TooLargeData;

pub trait TryToChunk {
    fn try_to_chunk(&self) -> Result<Chunk, TooLargeData>;
}

pub trait TryFromChunk
where Self: Sized
{
    type Error: std::error::Error;
    fn try_from_chunk(chunk: Chunk) -> Result<Self, Self::Error>;
}

/// Marker trait defining specific encoding strategy which should be used for
/// conversion into and from [`Chunk`] blob.
pub trait Strategy {
    /// Specific strategy. List of supported strategies:
    /// - [`StrictEncoding`]
    type Strategy;
}

pub mod encoding {
    use strict_encoding::MediumVec;
    pub use strict_encoding::{Error, StrictDecode, StrictEncode};

    use super::{Chunk, Strategy};
    use crate::chunk::{TooLargeData, TryFromChunk, TryToChunk};

    /// A marker trait for simple implementation of serialization into a
    /// [`Chunk`] using strict encoding for existing types, including foreign
    /// types.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate strict_encoding;
    /// # use storm::chunk;
    ///
    /// #[derive(StrictEncode, StrictDecode)]
    /// struct Type {}
    ///
    /// impl chunk::encoding::ApplyStrictEncoding for Type {}
    /// ```
    pub trait ApplyStrictEncoding {}

    impl<T> Strategy for T
    where T: ApplyStrictEncoding
    {
        type Strategy = UseStrictEncoding;
    }

    /// Encodes/decodes data in the same way as they are encoded/decoded
    /// according to strict encoding.
    pub struct UseStrictEncoding;

    impl<T> TryToChunk for T
    where
        T: Strategy + Clone,
        amplify::Holder<T, <T as Strategy>::Strategy>: TryToChunk,
    {
        fn try_to_chunk(&self) -> Result<Chunk, TooLargeData> {
            amplify::Holder::new(self.clone()).try_to_chunk()
        }
    }

    impl<T> TryFromChunk for T
    where
        T: Strategy,
        amplify::Holder<T, <T as Strategy>::Strategy>: TryFromChunk,
    {
        type Error = <amplify::Holder<T, <T as Strategy>::Strategy> as TryFromChunk>::Error;

        fn try_from_chunk(chunk: Chunk) -> Result<T, Self::Error> {
            amplify::Holder::try_from_chunk(chunk)
                .map(amplify::Holder::into_inner)
        }
    }

    impl<B> TryToChunk for amplify::Holder<B, UseStrictEncoding>
    where B: StrictEncode
    {
        fn try_to_chunk(&self) -> Result<Chunk, TooLargeData> {
            self.as_inner()
                .strict_serialize()
                .and_then(MediumVec::try_from)
                .map(Chunk::from)
                .map_err(|_| TooLargeData)
        }
    }

    impl<B> TryFromChunk for amplify::Holder<B, UseStrictEncoding>
    where B: StrictDecode
    {
        type Error = Error;

        fn try_from_chunk(chunk: Chunk) -> Result<Self, Self::Error> {
            B::strict_deserialize(chunk).map(amplify::Holder::new)
        }
    }
}

#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From, Display
)]
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

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = strict_encoding::Error;

    fn try_from(vec: &Vec<u8>) -> Result<Self, Self::Error> {
        MediumVec::try_from(vec.to_vec()).map(Self)
    }
}

impl TryFrom<Vec<u8>> for Chunk {
    type Error = strict_encoding::Error;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        MediumVec::try_from(vec).map(Self)
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

impl TryToChunk for Chunk {
    fn try_to_chunk(&self) -> Result<Chunk, TooLargeData> { Ok(self.clone()) }
}

impl TryFromChunk for Chunk {
    type Error = Infallible;

    fn try_from_chunk(chunk: Chunk) -> Result<Self, Self::Error> { Ok(chunk) }
}
