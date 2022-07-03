// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::BTreeSet;

use internet2::{CreateUnmarshaller, Unmarshaller};
use once_cell::sync::Lazy;

use crate::mesg::Topic;
use crate::{Chunk, ChunkId, Container, ContainerId, Mesg, MesgId, StormApp};

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
    /// List Storm apps supported by the node.
    #[display("list_apps")]
    #[api(type = 0x0002)]
    ListApps,

    /// List of Storm apps registered with the node for public announcement.
    #[display("active_apps(...)")]
    #[api(type = 0x0003)]
    ActiveApps(BTreeSet<StormApp>),

    /// List topics under the specified app.
    #[display("list_topics({0})")]
    #[api(type = 0x0004)]
    ListTopics(StormApp),

    /// Response to `ListTopics` request.
    #[api(type = 0x0005)]
    AppTopics(TopicList),

    /// Propose to create a new Storm application topic.
    #[display("propose_topic({0})")]
    #[api(type = 0x0006)]
    ProposeTopic(Topic),

    /// Post a message under specific app and topic from one peer to another.
    /// Can be a reply to `Read` message or a spontaneous message, which will
    /// require reply in form of `Accept` or `Decline` messages.
    #[api(type = 0x0008)]
    Post(PostReq),

    /// Read a message or a topic from an app.
    #[api(type = 0x000a)]
    #[display("read({0})")]
    Read(MesgReq),

    #[api(type = 0x000c)]
    #[display("decline({0})")]
    Decline(MesgReq),

    #[api(type = 0x000e)]
    #[display("accept({0})")]
    Accept(MesgReq),

    /// Request to obtain container information.
    #[api(type = 0x0010)]
    PullContainer(ContainerPull),

    /// Response on container pull request providing with the container information (chunks, mime
    /// etc).
    #[api(type = 0x0011)]
    PushContainer(ContainerPush),

    /// Reject to provide the container
    #[api(type = 0x0012)]
    #[display("reject({0})")]
    Reject(ContainerPull),

    /// Pull a chunk data from a peer, if they are known to it.
    #[api(type = 0x0014)]
    PullChunk(ChunkPull),

    /// Response to a chunk pull request, providing source data.
    #[api(type = 0x0015)]
    PushChunk(ChunkPush),
}

impl StormMesg for Messages {
    fn storm_app(&self) -> StormApp {
        match self {
            Messages::ListApps => StormApp::System,
            Messages::ActiveApps(_) => StormApp::System,
            Messages::ListTopics(app) => *app,
            Messages::AppTopics(msg) => msg.storm_app(),
            Messages::ProposeTopic(msg) => msg.storm_app(),
            Messages::Accept(msg) => msg.storm_app(),
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

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("topic_list({app}, ...)")]
pub struct TopicList {
    pub app: StormApp,
    pub topics: BTreeSet<MesgId>,
}

impl StormMesg for TopicList {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("post({app}, {message})")]
pub struct PostReq {
    pub app: StormApp,
    pub message: Mesg,
}

impl StormMesg for PostReq {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("{app}, {message_id}")]
pub struct MesgReq {
    pub app: StormApp,
    pub message_id: MesgId,
}

impl StormMesg for MesgReq {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("container_pull({app}, {message_id}, {container_id})")]
pub struct ContainerPull {
    pub app: StormApp,
    /// Message defining access rights to the container.
    pub message_id: MesgId,
    pub container_id: ContainerId,
}

impl StormMesg for ContainerPull {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("container_push({app}, {container_id}, ...)")]
pub struct ContainerPush {
    pub app: StormApp,
    pub container_id: ContainerId,
    pub container: Container,
}

impl StormMesg for ContainerPush {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("chunk_pull({app}, {message_id}, {container_id}, ...)")]
pub struct ChunkPull {
    pub app: StormApp,
    pub message_id: MesgId,
    pub container_id: ContainerId,
    pub chunk_ids: BTreeSet<ChunkId>,
}

impl StormMesg for ChunkPull {
    fn storm_app(&self) -> StormApp { self.app }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, NetworkEncode, NetworkDecode)]
#[display("chunk_push({app}, {container_id}, {chunk_id}, ...)")]
pub struct ChunkPush {
    pub app: StormApp,
    pub container_id: ContainerId,
    pub chunk_id: ChunkId,
    pub chunk: Chunk,
}

impl StormMesg for ChunkPush {
    fn storm_app(&self) -> StormApp { self.app }
}
