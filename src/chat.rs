use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Chat {
    pub text: String,
    pub player_name: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "event_type")]
pub enum Message {
    #[serde(rename = "chat")]
    Chat(Chat),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_console_text() {
        let text = r#"{"event_type":"chat","tick":831674,"text":"message"}"#;

        let message: Message = serde_json::from_str(text).unwrap();

        match message {
            Message::Chat(event) => {
                assert_eq!(event.text, "message");
            }
        }
    }

    #[test]
    fn it_parse_chat_text() {
        let text =
            r#"{"event_type":"chat","tick":831674,"text":"message", "player_name": "whoopi"}"#;

        let message: Message = serde_json::from_str(text).unwrap();

        match message {
            Message::Chat(event) => {
                assert_eq!(event.text, "message");
                assert_eq!(event.player_name, Some("whoopi".into()));
            }
        }
    }
}
