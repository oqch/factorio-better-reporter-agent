use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Reason {
    Quit = 0,
    Dropped = 1,
    Reconnect = 2,
    WrongInput = 3,
    DesyncLimitReached = 4,
    CannotKeepUp = 5,
    Afk = 6,
    Kicked = 7,
    KickedAndDeleted = 8,
    Banned = 9,
    SwitchingServers = 10,
    Unknown,
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Reason::Quit => "",
            Reason::Dropped => "(ドロップ)",
            Reason::Reconnect => "(再接続)",
            Reason::WrongInput => "(不正な入力)",
            Reason::DesyncLimitReached => "(同期タイムアウト)",
            Reason::CannotKeepUp => "(同期継続失敗)",
            Reason::Afk => "(AFK)",
            Reason::Kicked => "(キック）",
            Reason::KickedAndDeleted => "(キック&削除)",
            Reason::Banned => "(バン)",
            Reason::SwitchingServers => "(サーバの切替)",
            Reason::Unknown => "(不明)",
        };

        write!(f, "{}", text)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Join {
    pub player_name: String,
}

impl Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}が出勤しました。", self.player_name)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Left {
    pub player_name: String,
    pub reason: Reason,
}

impl Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}が退勤しました。{}", self.player_name, self.reason)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "event_type")]
pub enum Message {
    #[serde(rename = "player-joined-game")]
    Join(Join),
    #[serde(rename = "player-left-game")]
    Left(Left),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_join_text() {
        let text = r#"{"event_type":"player-joined-game","tick":831674,"player_name":"whoopi"}"#;

        let message: Message = serde_json::from_str(text).unwrap();

        match message {
            Message::Join(event) => {
                assert_eq!(event.player_name, "whoopi");
            }
            _ => panic!("can't parse join message"),
        }
    }

    #[test]
    fn it_parse_left_text() {
        let text =
            r#"{"event_type":"player-left-game","tick":831674,"player_name":"whoopi","reason":6}"#;

        let message: Message = serde_json::from_str(text).unwrap();

        match message {
            Message::Left(event) => {
                assert_eq!(event.player_name, "whoopi");
                assert_eq!(event.reason, Reason::Afk);
            }
            _ => panic!("can't parse join message"),
        }
    }

    #[test]
    fn it_converts_join_to_string() {
        let event = Join {
            player_name: "whoopi".into(),
        };
        assert_eq!(event.to_string(), "whoopiが出勤しました。");
    }

    #[test]
    fn it_converts_left_to_string() {
        let event = Left {
            player_name: "whoopi".into(),
            reason: Reason::Quit,
        };
        assert_eq!(event.to_string(), "whoopiが退勤しました。");

        let event = Left {
            player_name: "whoopi".into(),
            reason: Reason::Afk,
        };
        assert_eq!(event.to_string(), "whoopiが退勤しました。(AFK)");
    }
}
