use crate::types::events::*;
use serde::Serialize;
use std::clone::Clone;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::broadcast::*;

/** This trait exists to send different messages
 *
 */
pub trait ChannelQueue: Send + Sync {
    fn send(&self, value: Message) -> Result<usize, SendError<Message>>;
    fn subscribe(&self) -> Receiver<Message>;
}

#[derive(Clone, Debug, Serialize)]
pub struct SerializableEventData {
    #[serde(flatten)]
    inner: Box<dyn EventData>,
}

impl SerializableEventData {
    pub fn new(inner: Box<dyn EventData>) -> Self {
        Self { inner }
    }
}

//impl<T> SerializableEventData for T where T: EventData + Serialize {}
//impl<T> SerializableEventData for T where T: EventData + Serialize {}

#[derive(Clone, Debug)]
pub struct Message {
    pub value: Vec<SerializableEventData>,
    //pub value: Vec<Box<dyn EventData + Send + Sync + Serialize>>,
    //    pub value: Vec<Box<dyn EventData>>,
}

pub struct ChannelQueueImpl {
    pub tx: Sender<Message>,
}
impl ChannelQueue for ChannelQueueImpl {
    fn send(&self, value: Message) -> Result<usize, SendError<Message>> {
        self.tx.send(value)
    }

    fn subscribe(&self) -> Receiver<Message> {
        self.tx.subscribe()
    }
}
