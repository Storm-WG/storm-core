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

use crate::{Chunk, Container, Mesg, MesgId, StormApp};

pub static STORM_P2P_UNMARSHALLER: Lazy<Unmarshaller<Messages>> =
    Lazy::new(|| Messages::create_unmarshaller());

pub trait StormMesg {
    fn storm_app(&self) -> StormApp;
}

#[derive(Clone, Debug, Display, Api, NetworkEncode, NetworkDecode)]
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
    Decline(DeclineResp),
}

impl StormMesg for Messages {
    fn storm_app(&self) -> StormApp {
        match self {
            Messages::Post(msg) => msg.storm_app(),
            Messages::Read(msg) => msg.storm_app(),
            Messages::Push(msg) => msg.storm_app(),
            Messages::Chunk(msg) => msg.storm_app(),
            Messages::Decline(msg) => msg.storm_app(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("post({app}, {message})")]
pub struct PostReq {
    pub app: StormApp,
    pub message: Mesg,
    pub container: Option<Container>,
}

impl StormMesg for PostReq {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("read({app}, {message_id}, {with_container})")]
pub struct ReadReq {
    pub app: StormApp,
    pub message_id: MesgId,
    pub with_container: bool,
}

impl StormMesg for ReadReq {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("push({app}, ...)")]
pub struct ChunkPush {
    pub app: StormApp,
    pub chunk: Chunk,
}

impl StormMesg for ChunkPush {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("chunk({app}, {chunk_id})")]
pub struct ChunkPoll {
    pub app: StormApp,
    pub chunk_id: Chunk,
}

impl StormMesg for ChunkPoll {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("decline({app}, {mesg_id})")]
pub struct DeclineResp {
    pub app: StormApp,
    pub mesg_id: MesgId,
}

impl StormMesg for DeclineResp {
    fn storm_app(&self) -> StormApp { self.app }
}
