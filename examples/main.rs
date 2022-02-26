use anyhow::Result;
use rcon_rs::{Client, PacketType};

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");
    // create new connect using ip and port
    let mut conn = Client::new("127.0.0.1", "25575").await;
    // you MUST auth the connection before attempting to use it
    conn.auth("password").await?;
    // send any command you would like, the packet type is option and inferred to be a command by
    // default
    conn.send("say hi", Some(PacketType::Cmd)).await?;
    println!("done");
    Ok(())
}

