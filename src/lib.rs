// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;
#[macro_use]
extern crate internet2;

pub mod chunk;
pub mod p2p;
mod container;
mod mesg;
mod app;

pub use app::{
    StormApp, STORM_APP_CHAT, STORM_APP_RGB_CONTRACTS, STORM_APP_RGB_TRANSFERS,
    STORM_APP_SEARCH, STORM_APP_STORAGE, STORM_APP_SYSTEM,
    STORM_APP_VENDOR_MASK,
};
pub use chunk::{
    Chunk, ChunkFullId, ChunkId, ChunkIdExt, TryFromChunk, TryToChunk,
};
pub use container::{
    Container, ContainerFullId, ContainerHeader, ContainerId, ContainerInfo,
    STORM_CONTAINER_ID_HRP,
};
pub use mesg::{Mesg, MesgId, Topic};
