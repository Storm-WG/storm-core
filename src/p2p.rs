// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![allow(clippy::clone_on_copy)]

use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};

use internet2::{CreateUnmarshaller, Unmarshaller};
use once_cell::sync::Lazy;
use strict_encoding::{StrictDecode, StrictEncode};

use crate::container::ContainerFullId;
use crate::mesg::Topic;
use crate::{
    Chunk, ChunkId, Container, ContainerId, ContainerInfo, Mesg, MesgId,
    StormApp,
};

pub static STORM_P2P_UNMARSHALLER: Lazy<Unmarshaller<Messages>> =
    Lazy::new(Messages::create_unmarshaller);

pub trait StormMesg {
    fn storm_app(&self) -> StormApp;
}

#[derive(Clone, Debug, Display, Api, NetworkEncode, NetworkDecode)]
#[api(encoding = "strict")]
#[non_exhaustive]
pub enum Messages {
    /// List Storm apps supported by the node.
    #[display("list_apps")]
    #[api(type = 0x0002)]
    ListApps,

    /// List of Storm apps registered with the node for public announcement.
    #[display("active_apps(...)")]
    #[api(type = 0x0003)]
    ActiveApps(BTreeSet<StormApp>),

    /// List topics under the specified app.
    #[display("list_topics(...)")]
    #[api(type = 0x0004)]
    ListTopics(AppMsg<()>),

    /// Response to `ListTopics` request.
    #[api(type = 0x0005)]
    #[display("app_topics(...)")]
    AppTopics(AppMsg<BTreeSet<MesgId>>),

    /// Propose to create a new Storm application topic.
    #[display("propose_topic(...)")]
    #[api(type = 0x0006)]
    ProposeTopic(AppMsg<Topic>),

    /// Post a message under specific app and topic from one peer to another.
    /// Can be a reply to `Read` message or a spontaneous message, which will
    /// require reply in form of `Accept` or `Decline` messages.
    #[api(type = 0x0008)]
    #[display("post({0})")]
    Post(AppMsg<Mesg>),

    /// Read a message or a topic from an app.
    #[api(type = 0x000a)]
    #[display("read({0})")]
    Read(AppMsg<MesgId>),

    #[api(type = 0x000c)]
    #[display("decline({0})")]
    Decline(AppMsg<MesgId>),

    #[api(type = 0x000e)]
    #[display("accept({0})")]
    Accept(AppMsg<MesgId>),

    // TODO: Consider using Storm mesgs for this
    /// Announce container.
    #[api(type = 0x0011)]
    #[display("announce_container({0})")]
    AnnounceContainer(AppMsg<ContainerInfo>),

    /// Request to obtain container information.
    #[api(type = 0x0010)]
    #[display("pull_container({0})")]
    PullContainer(AppMsg<ContainerFullId>),

    /// Response on container pull request providing with the container
    /// information (chunks, mime etc).
    #[api(type = 0x0013)]
    #[display("push_container(...)")]
    PushContainer(AppMsg<Container>),

    /// Reject to provide the container
    #[api(type = 0x0012)]
    #[display("reject({0})")]
    Reject(AppMsg<ContainerFullId>),

    /// Pull a chunk data from a peer, if they are known to it.
    #[api(type = 0x0014)]
    #[display("pull_chunk({0})")]
    PullChunk(ChunkPull),

    /// Response to a chunk pull request, providing source data.
    #[api(type = 0x0015)]
    #[display("push_chunk({0})")]
    PushChunk(ChunkPush),
}

impl StormMesg for Messages {
    fn storm_app(&self) -> StormApp {
        match self {
            Messages::ListApps => StormApp::System,
            Messages::ActiveApps(_) => StormApp::System,
            Messages::ListTopics(msg) => msg.storm_app(),
            Messages::AppTopics(msg) => msg.storm_app(),
            Messages::ProposeTopic(msg) => msg.storm_app(),
            Messages::Accept(msg) => msg.storm_app(),
            Messages::AnnounceContainer(msg) => msg.storm_app(),
            Messages::PullContainer(msg) => msg.storm_app(),
            Messages::PushContainer(msg) => msg.storm_app(),
            Messages::Post(msg) => msg.storm_app(),
            Messages::Read(msg) => msg.storm_app(),
            Messages::PushChunk(msg) => msg.storm_app(),
            Messages::PullChunk(msg) => msg.storm_app(),
            Messages::Decline(msg) => msg.storm_app(),
            Messages::Reject(msg) => msg.storm_app(),
        }
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(NetworkEncode, NetworkDecode)]
pub struct AppMsg<T>
where T: StrictEncode + StrictDecode
{
    pub app: StormApp,
    pub data: T,
}

impl<T> StormMesg for AppMsg<T>
where T: StrictEncode + StrictDecode
{
    fn storm_app(&self) -> StormApp { self.app }
}

impl<T> Display for AppMsg<T>
where T: Display + StrictEncode + StrictDecode
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.app, self.data)
    }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{app}, {message_id}, {container_id}, ...")]
pub struct ChunkPull {
    pub app: StormApp,
    pub message_id: MesgId,
    pub container_id: ContainerId,
    pub chunk_ids: BTreeSet<ChunkId>,
}

impl StormMesg for ChunkPull {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{app}, {container_id}, {chunk_id}, ...")]
pub struct ChunkPush {
    pub app: StormApp,
    pub container_id: ContainerId,
    pub chunk_id: ChunkId,
    pub chunk: Chunk,
}

impl StormMesg for ChunkPush {
    fn storm_app(&self) -> StormApp { self.app }
}
