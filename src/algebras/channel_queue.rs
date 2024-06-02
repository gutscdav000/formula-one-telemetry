use crate::types::event::*;
use log::error;
use std::clone::Clone;
use std::fmt;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::broadcast::*;

/** This trait exists to send messages of different types.
 *  The current implementation only Sends a Message<Event>, but the intent is to
 *  eventually have the option to send the data types associated with API data
 */
pub trait ChannelQueue: Send + Sync {
    fn send(&self, value: Message) -> Result<usize, SendError<Message>>;
    fn subscribe(&self) -> Receiver<Message>;
    fn fire_and_forget(&self, event: Event);
}

#[derive(Clone, Debug)]
pub struct Message {
    pub msg: Event,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.msg)
    }
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

    fn fire_and_forget(&self, event: Event) {
        self.tx
            .send(Message { msg: event })
            .err()
            .map(|e| error!("failed to send Events message: {e}"));
    }
}
