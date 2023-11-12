use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::services::text_helper::tokenized_text;
use crate::telegram::request::WebhookRequest;

// TODO: Перенести из проекта bread_bot все команды в структуру Command.
#[derive(Debug, PartialEq, EnumIter)]
enum Command {
    Help,
    Who,
    AnswerChance,
}

impl Command {
    fn key_words(&self) -> Vec<&str> {
        use Command::*;
        match &self {
            Help => vec!["хелп", "help", "помощь"],
            Who => vec!["кто", "who"],
            AnswerChance => vec!["процент срабатывания", "процент"]
        }
    }
}

// TODO: Переписать функцию на to_command, с матчингом в Option<Command>
pub fn is_command(request: WebhookRequest) -> bool {
    if let Some(text_token) = tokenized_text(request.any_message_request().direct().ext.raw_text()) {
        text_token[0][0] == "хлеб"
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::services::commands::is_command;
    use crate::telegram::request::WebhookRequest;

    #[test]
    fn test_with_caption() {
        let message: Result<WebhookRequest, _> = serde_json::from_str(r#"{
  "update_id": 959895795,
  "edited_message": {
    "message_id": 104043,
    "from": {
      "id": 211382611,
      "is_bot": false,
      "first_name": "FirstName",
      "last_name": "LastName",
      "username": "UserName",
      "language_code": "en"
    },
    "chat": {
      "id": 211382611,
      "first_name": "FirstName",
      "last_name": "LastName",
      "username": "UserName",
      "type": "private"
    },
    "date": 1699406048,
    "edit_date": 1699406157,
    "animation": {
      "file_name": "scrubs-dr-perrycox.mp4",
      "mime_type": "video/mp4",
      "duration": 2,
      "width": 220,
      "height": 220,
      "thumbnail": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "thumb": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "file_id": "CgACAgQAAxkBAAEBlmtlSuFNwDJT6vuXpvxNlwZ2EdUTeAACDAMAAuTuDVMjrKeBdG069DME",
      "file_unique_id": "AgADDAMAAuTuDVM",
      "file_size": 49283
    },
    "document": {
      "file_name": "scrubs-dr-perrycox.mp4",
      "mime_type": "video/mp4",
      "thumbnail": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "thumb": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "file_id": "CgACAgQAAxkBAAEBlmtlSuFNwDJT6vuXpvxNlwZ2EdUTeAACDAMAAuTuDVMjrKeBdG069DME",
      "file_unique_id": "AgADDAMAAuTuDVM",
      "file_size": 49283,
      "caption": "some_caption"
    },
    "caption": "хлеб кто чайник?\n хы"
  }
}"#);
        assert!(is_command(message.unwrap()));
    }

    #[test]
    fn test_without_caption() {
        let message_without_caption: Result<WebhookRequest, _> = serde_json::from_str(r#"{
  "update_id": 959895795,
  "edited_message": {
    "message_id": 104043,
    "from": {
      "id": 211382611,
      "is_bot": false,
      "first_name": "FirstName",
      "last_name": "LastName",
      "username": "UserName",
      "language_code": "en"
    },
    "chat": {
      "id": 211382611,
      "first_name": "FirstName",
      "last_name": "LastName",
      "username": "UserName",
      "type": "private"
    },
    "date": 1699406048,
    "edit_date": 1699406157,
    "animation": {
      "file_name": "scrubs-dr-perrycox.mp4",
      "mime_type": "video/mp4",
      "duration": 2,
      "width": 220,
      "height": 220,
      "thumbnail": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "thumb": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "file_id": "CgACAgQAAxkBAAEBlmtlSuFNwDJT6vuXpvxNlwZ2EdUTeAACDAMAAuTuDVMjrKeBdG069DME",
      "file_unique_id": "AgADDAMAAuTuDVM",
      "file_size": 49283
    },
    "document": {
      "file_name": "scrubs-dr-perrycox.mp4",
      "mime_type": "video/mp4",
      "thumbnail": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "thumb": {
        "file_id": "AAMCBAADGQEAAQGWa2VK4U3AMlPq-5em_E2XBnYR1RN4AAIMAwAC5O4NUyOsp4F0bTr0AQAHbQADMwQ",
        "file_unique_id": "AQADDAMAAuTuDVNy",
        "file_size": 7292,
        "width": 220,
        "height": 220
      },
      "file_id": "CgACAgQAAxkBAAEBlmtlSuFNwDJT6vuXpvxNlwZ2EdUTeAACDAMAAuTuDVMjrKeBdG069DME",
      "file_unique_id": "AgADDAMAAuTuDVM",
      "file_size": 49283,
      "caption": "some_caption"
    }
  }
}"#);
        assert!(!is_command(message_without_caption.unwrap()));
    }
}