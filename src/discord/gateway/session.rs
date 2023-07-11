use std::{error::Error, time};


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
        let (_, mut read) = stream.split();

        while let Some(msg) = read.next().await {
            if let Message::Text(text) = msg.unwrap() {
                let e = from_slice::<Event>(text.as_bytes())?;
                self.handle_event(e).await;
            }
        }

        Ok(())
    }

    async fn handle_event(&self, e: Event) {
        match e {
            Event::Hello(heartbeat_interval) => {
                let mut interval =
                    tokio::time::interval(time::Duration::from_millis(heartbeat_interval));

                loop {
                    interval.tick().await;
                    self.send_heartbeat().await;
                }
            }
        }
    }

    async fn send_heartbeat(&self) {}
}
