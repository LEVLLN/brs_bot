use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq)]
pub struct LinkPreviewOption {
    pub is_disabled: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyMarkupButtonResponse {
    pub text: String,
    pub callback_data: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplyMarkupResponse {
    pub inline_keyboard: Vec<Vec<ReplyMarkupButtonResponse>>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct BaseBody {
    pub chat_id: i64,
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkupResponse>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseMessage {
    Text {
        #[serde(flatten)]
        base_body: BaseBody,
        text: String,
        link_preview_options: LinkPreviewOption,
    },
    Photo {
        #[serde(flatten)]
        base_body: BaseBody,
        photo: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    Video {
        #[serde(flatten)]
        base_body: BaseBody,
        video: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    Voice {
        #[serde(flatten)]
        base_body: BaseBody,
        voice: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    Audio {
        #[serde(flatten)]
        base_body: BaseBody,
        audio: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    Document {
        #[serde(flatten)]
        base_body: BaseBody,
        document: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    Animation {
        #[serde(flatten)]
        base_body: BaseBody,
        animation: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
    },
    VideoNote {
        #[serde(flatten)]
        base_body: BaseBody,
        video_note: String,
    },
    Sticker {
        #[serde(flatten)]
        base_body: BaseBody,
        sticker: String,
    },
}

pub fn roll_reply_markup() -> Option<ReplyMarkupResponse> {
    Some(ReplyMarkupResponse {
        inline_keyboard: vec![vec![ReplyMarkupButtonResponse {
            text: "Roll".to_string(),
            callback_data: "".to_string(),
        }]],
    })
}

pub fn text_message(
    value: String,
    chat_id: i64,
    reply_to_message_id: i64,
) -> ResponseMessage {
    ResponseMessage::Text {
        base_body: BaseBody {
            chat_id,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: None,
        },
        text: value,
        link_preview_options: LinkPreviewOption { is_disabled: false },
    }
}

pub fn text_message_with_roll(
    value: String,
    chat_id: i64,
    reply_to_message_id: i64,
) -> ResponseMessage {
    ResponseMessage::Text {
        base_body: BaseBody {
            chat_id,
            reply_to_message_id: Some(reply_to_message_id),
            reply_markup: roll_reply_markup()
        },
        text: value,
        link_preview_options: LinkPreviewOption { is_disabled: false },
    }
}