use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;

/// A Single Actor
pub struct Actor {
    /// The channel it receives messages through
    pub receiver: Receiver<ActorMessage>,
    /// The next message id
    pub next_id: u32,
}

/// The Actor Message
pub enum ActorMessage {
  Id(oneshot::Sender<u32>),
}

impl Actor {
  /// Instantiates a new actor
  pub fn new(receiver: Receiver<ActorMessage>) -> Self {
    Self {
      receiver,
      next_id: 0,
    }
  }

  /// The Actor's message handling
  pub fn handle_message(&mut self, msg: ActorMessage) {
    match msg {
      ActorMessage::Id(responder) => {
        self.next_id += 1;
        let _ = responder.send(self.next_id);
      }
    }
  }

  /// Starts the Actor
  pub async fn start(&mut self) {
    while let Some(msg) = self.receiver.recv().await {
      self.handle_message(msg);
    }
  }
}

