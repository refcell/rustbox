use tokio::sync::{mpsc::{Sender, self}, oneshot};

use crate::actor::{ActorMessage, Actor};


pub struct ActorHandler {
  /// The channel to send messages to the actor
  pub sender: Sender<ActorMessage>,
}

impl ActorHandler {
  /// Instantiates a new Actor Handler
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel(8);
    let mut actor = Actor::new(receiver);
    tokio::spawn(async move { actor.start().await });
    Self { sender }
  }

  /// Get a unique id
  pub async fn get_id(&self) -> u32 {
    let (tx, rx) = oneshot::channel();
    let _ = self.sender.send(ActorMessage::Id(tx)).await;
    rx.await.unwrap()
  }
}