use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub is_bot: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub id: u32,
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
pub enum Message {
    Photo {
        #[serde(flatten)]
        base: MessageBase,
        photo: Vec<Content>,
        caption: Option<String>,
    },
    Text {
        #[serde(flatten)]
        base: MessageBase,
        text: String,
    },
    Video {
        #[serde(flatten)]
        base: MessageBase,
        video: Content,
        caption: Option<String>,
    },
    Voice {
        #[serde(flatten)]
        base: MessageBase,
        voice: Content,
        caption: Option<String>,
    },
    VideoNote {
        #[serde(flatten)]
        base: MessageBase,
        video_note: Content,
    },
    Sticker {
        #[serde(flatten)]
        base: MessageBase,
        sticker: Content,
    },
    Animation {
        #[serde(flatten)]
        base: MessageBase,
        animation: Content,
        caption: Option<String>,
    },
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageRequest {
    Replied {
        #[serde(flatten)]
        message: Message,
        reply_to_message: Message,
    },
    Common(Message),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WebhookRequest {
    Edited { update_id: u32, edited_message: MessageRequest },
    Origin { update_id: u32, message: MessageRequest },
}
