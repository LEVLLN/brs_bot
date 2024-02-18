use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub id: i64,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Content {
    pub file_id: String,
    pub file_unique_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageBase {
    pub message_id: u32,
    pub from: User,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageExt {
    Photo {
        photo: Vec<Content>,
        caption: Option<String>,
    },
    Text {
        text: String,
    },
    Video {
        video: Content,
        caption: Option<String>,
    },
    Voice {
        voice: Content,
        caption: Option<String>,
    },
    VideoNote {
        video_note: Content,
    },
    Sticker {
        sticker: Content,
    },
    Animation {
        animation: Content,
        caption: Option<String>,
    },
}

impl MessageExt {
    pub fn raw_text(&self) -> Option<&str> {
        use MessageExt::*;
        match &self {
            Photo { caption, .. }
            | Video { caption, .. }
            | Voice { caption, .. }
            | Animation { caption, .. } => caption.as_deref(),
            Text { text, .. } => Some(text),
            VideoNote { .. } | Sticker { .. } => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageBody {
    #[serde(flatten)]
    pub base: MessageBase,
    #[serde(flatten)]
    pub ext: MessageExt,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Message {
    Common{
        #[serde(flatten)]
        direct: MessageBody
    },
    Replied{
        #[serde(flatten)]
        direct: MessageBody,
        #[serde(alias = "reply_to_message")]
        reply: Box<MessageBody>,
    }
}

impl Message {
    pub fn direct(&self) -> &MessageBody {
        match &self {
            Message::Common {direct} | Message::Replied {direct, ..} => direct
        }
    }
    pub fn reply(&self) -> Option<&MessageBody> {
        if let Message::Replied {reply, ..} = self {
            Some(reply)
        }
        else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RequestPayload {
    Edited {
        update_id: u32,
        edited_message: Message,
    },
    Origin {
        update_id: u32,
        message: Message,
    },
}

impl RequestPayload {
    pub fn any_message(&self) -> &Message {
        match self {
            RequestPayload::Edited { edited_message, .. } => edited_message,
            RequestPayload::Origin { message, .. } => message,
        }
    }
}