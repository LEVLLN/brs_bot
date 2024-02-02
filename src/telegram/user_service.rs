use log::{info, warn};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgQueryResult;

use crate::telegram::db::{Chat as ChatDB, Member as MemberDB};
use crate::telegram::request::{Chat as ChatRequest, User};

#[derive(Debug, PartialEq)]
pub struct UserError<'a> {
    message: &'a str,
}


pub fn chat_title(chat: &ChatRequest) -> String {
    match chat {
        ChatRequest {
            title: Some(title), ..
        } if !title.is_empty() => title.to_string(),
        ChatRequest {
            username: Some(username),
            ..
        } if !username.is_empty() => username.to_string(),
        ChatRequest {
            first_name: Some(first_name),
            last_name: Some(last_name),
            ..
        } if !first_name.is_empty() || !last_name.is_empty() => {
            format!("{first_name} {last_name}").to_string()
        }
        _ => chat.id.to_string(),
    }
}

pub async fn process_chat<'a>(
    pool: &Pool<Postgres>,
    chat: &ChatRequest,
) -> Result<u64, UserError<'a>> {
    let new_chat_name = chat_title(chat);
    if let Some(chat_name) = ChatDB::chat_name(pool, &chat.id).await {
        if new_chat_name != chat_name {
            match ChatDB::update_name(pool, &chat.id, &new_chat_name).await {
                Ok(res) => {info!("Chat {} name updated from: {} to: {}",&chat.id, chat_name, new_chat_name); Ok(res.rows_affected())}
                Err(err) => {warn!("Chat {} update name error: {}", &chat.id, err.to_string()); Err(UserError {message: "Не удалось обновить имя чата"})}
            }
        } else {
            Ok(0)
        }
    } else {
        match ChatDB::create_chat(&pool, &chat.id, &new_chat_name).await {
            Ok(res) => {info!("Chat {} created with name: {}",&chat.id, new_chat_name); Ok(res.rows_affected())},
            Err(err) => {warn!("Chat {} creating error: {}", &chat.id, err.to_string()); Err(UserError {message: "Не удалось создать чат"})},
        }
    }
}

pub async fn process_user<'a>(pool: &Pool<Postgres>, user: &User) -> Result<Option<PgQueryResult>, UserError<'a>> {
    if user.is_bot {
        return Err(UserError{message: "Работа с ботами исключена"})
    }
    MemberDB::one_by_member_id(pool, &user.id).await;
    Ok(None)
}