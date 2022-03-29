use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create connection to the mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get hello key
    let value = client.get("hello").await?;

    println!("{:?}", value);

    Ok(())
}
