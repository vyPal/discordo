use serde::{de::Error, Deserialize, Serialize};
use serde_json::Map;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Opcode {
    Hello = 10,
}

#[derive(Debug, Serialize)]
pub enum Event {
    Hello(u64),
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = Map::deserialize(deserializer)?;

        let op = map
            .remove("op")
            .ok_or_else(|| Error::missing_field("op"))
            .and_then(Opcode::deserialize)
            .map_err(Error::custom)?;

        Ok(match op {
            Opcode::Hello => {
                let mut d = map
                    .remove("d")
                    .ok_or_else(|| Error::missing_field("d"))
                    .and_then(Map::deserialize)
                    .map_err(Error::custom)?;

                let heartbeat_interval = d
                    .remove("heartbeat_interval")
                    .ok_or_else(|| Error::missing_field("heartbeat_interval"))
                    .and_then(u64::deserialize)
                    .map_err(Error::custom)?;

                Event::Hello(heartbeat_interval)
            }
        })
    }
}
