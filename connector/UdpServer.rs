use tokio::net::UdpSocket;
use std::io;
use mini_redis::{client, Result};

#[tokio::main]
pub async fn server_start() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("10.0.1.15:6379").await?;

    let test = client.get("hello").await?;
    if test != Some("world".into()) {
      // Set the key "hello" with value "world"
      client.set("hello", "world".into()).await?;
    }else{
	client.set("hello","picahundakunda".into()).await?;
    }

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
