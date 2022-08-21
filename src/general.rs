use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PlayerDied {
    pub player_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "event_type")]
pub enum Message {
    #[serde(rename = "player-died")]
    PlayerDied(PlayerDied),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_join_text() {
        let text = r#"{"event_type":"player-died","tick":831674,"player_name":"whoopi"}"#;

        let message: Message = serde_json::from_str(text).unwrap();

        match message {
            Message::PlayerDied(event) => {
                assert_eq!(event.player_name, "whoopi");
            }
        }
    }
}
