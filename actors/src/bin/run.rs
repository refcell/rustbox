use actors::handler;


#[tokio::main]
async fn main() {
  let handler = handler::ActorHandler::new();
  loop {
    let fetched_val = handler.get_id().await;
    println!("Handler Recv: [{}]", fetched_val);
  }
}