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
pub enum ResponseMessage<'a> {
    Text{
        #[serde(flatten)]
        base_body: BaseBody,
        text: &'a str,
        link_preview_options: Option<LinkPreviewOption>
    },
    Photo {
        #[serde(flatten)]
        base_body: BaseBody,
        photo: &'a str,
        caption: Option<&'a str>
    }
}
