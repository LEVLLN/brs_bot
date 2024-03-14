use sqlx::PgPool;

use crate::common::db::{AnswerEntity, ChatId, EntityReactionType};
use crate::common::lexer::{normalize_text, Token, tokens_to_string};

pub async fn substrings<'a>(pool: &PgPool, tokens: &'a [Token<'a>], chat_db_id: &ChatId) -> Vec<AnswerEntity> {
    let mut keys: Vec<String> = vec![];
    if tokens.len() > 1 {
        keys.push(normalize_text(tokens_to_string(tokens, false)));
    }
    tokens.iter().for_each(|x| {
        if let Token::Word(word) = x {
            keys.push(normalize_text(word.to_string()))
        }
    });
    AnswerEntity::find(pool, chat_db_id, &keys, &EntityReactionType::Substring).await
}

pub async fn triggers<'a>(pool: &PgPool, tokens: &'a [Token<'a>], chat_db_id: &ChatId) -> Vec<AnswerEntity> {
    AnswerEntity::find(pool, chat_db_id, &[normalize_text(tokens_to_string(tokens, false))], &EntityReactionType::Trigger).await
}
