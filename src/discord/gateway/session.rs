use std::error::Error;

use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const GATEWAY_URL: &str = "wss://gateway.discord.gg";

pub struct Session {
    token: String,
}

impl Session {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let (stream, _) = connect_async(GATEWAY_URL).await?;
        let (_, read) = stream.split();

        read.for_each(|msg| async {
            if let Message::Text(text) = msg.unwrap() {
                println!("{text:?}");
            }
        })
        .await;

        Ok(())
    }
}
