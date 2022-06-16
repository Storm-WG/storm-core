// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use internet2::{CreateUnmarshaller, Unmarshaller};
use once_cell::sync::Lazy;

use crate::{Chunk, Container, Mesg, MesgId};

pub static STORM_P2P_UNMARSHALLER: Lazy<Unmarshaller<Messages>> =
    Lazy::new(|| Messages::create_unmarshaller());

pub type AppId = u16;

#[derive(Clone, Debug, Display, Api)]
#[api(encoding = "strict")]
#[non_exhaustive]
#[display(inner)]
pub enum Messages {
    #[api(type = 0x0010)]
    Post(PostReq),

    #[api(type = 0x0011)]
    Read(ReadReq),

    #[api(type = 0x0012)]
    Push(ChunkPush),

    #[api(type = 0x0013)]
    Chunk(ChunkPoll),

    #[api(type = 0x0020)]
    Decline(MesgId),
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("post({app:#06X}, {message})")]
pub struct PostReq {
    pub app: AppId,
    pub message: Mesg,
    pub container: Option<Container>,
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("read({app:#06X}, {message_id}, {with_container})")]
pub struct ReadReq {
    pub app: AppId,
    pub message_id: MesgId,
    pub with_container: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("push({app:#06X}, ...)")]
pub struct ChunkPush {
    pub app: AppId,
    pub chunk: Chunk,
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("chunk({app:#06X}, {chunk_id})")]
pub struct ChunkPoll {
    pub app: AppId,
    pub chunk_id: Chunk,
}
