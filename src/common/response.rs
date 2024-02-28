use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct LinkPreviewOption {
    pub is_disabled: bool
}
#[derive(Debug, Serialize, PartialEq)]
pub struct BaseBody {
    pub chat_id: i64,
    pub reply_to_message_id: Option<i64>,
}


#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseMessage {
    Text{
        #[serde(flatten)]
        base_body: BaseBody,
        text: String,
        link_preview_options: LinkPreviewOption
    },
    _Photo {
        #[serde(flatten)]
        base_body: BaseBody,
        photo: String,
        caption: Option<String>
    }
}
