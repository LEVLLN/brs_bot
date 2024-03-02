use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct LinkPreviewOption {
    pub is_disabled: bool,
}
#[derive(Debug, Serialize, PartialEq)]
pub struct BaseBody {
    pub chat_id: i64,
    pub reply_to_message_id: Option<i64>,
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
    _Photo {
        #[serde(flatten)]
        base_body: BaseBody,
        photo: String,
        caption: Option<String>,
    },
}

pub fn text_message(
    text_message: String,
    chat_id: i64,
    reply_to_message_id: i64,
) -> ResponseMessage {
    ResponseMessage::Text {
        base_body: BaseBody {
            chat_id,
            reply_to_message_id: Some(reply_to_message_id),
        },
        text: text_message,
        link_preview_options: LinkPreviewOption { is_disabled: false },
    }
}
