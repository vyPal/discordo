use std::error::Error;

use futures::StreamExt;
use serde_json::from_slice;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::discord::gateway::Event;

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
                let e = from_slice::<Event>(text.as_bytes());
                println!("{e:#?}");
            }
        })
        .await;

        Ok(())
    }
}
