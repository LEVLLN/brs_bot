use log::{info, warn};
use sqlx::{Pool, Postgres};

use crate::telegram::db::{Chat as ChatDB, Member as MemberDB};
use crate::telegram::request::{Chat as ChatRequest, User as UserRequest};

#[derive(Debug, PartialEq)]
pub enum UserServiceError<'a> {
    UserBotUnable(&'a i64),
    UserUpdate(&'a i64),
    ChatNameUpdate(&'a i64),
    UserCreate(&'a i64),
    ChatCreate(&'a i64),
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

pub fn username(user: &UserRequest) -> String {
    match user {
        UserRequest {
            first_name: Some(first_name),
            last_name: Some(last_name),
            ..
        } if !first_name.is_empty() || !last_name.is_empty() => {
            format!("{first_name} {last_name}").to_string()
        }
        UserRequest {
            username: Some(username),
            ..
        } if !username.is_empty() => username.to_string(),
        _ => user.id.to_string(),
    }
}

pub async fn process_chat<'a>(
    pool: &Pool<Postgres>,
    chat: &'a ChatRequest,
) -> Result<(), UserServiceError<'a>> {
    let new_chat_name = chat_title(chat);
    if let Some(chat_name) = ChatDB::chat_name(pool, &chat.id).await {
        if new_chat_name != chat_name {
            match ChatDB::update_name(pool, &chat.id, &new_chat_name).await {
                Ok(_) => {
                    info!(
                        "Chat {} name updated from: {} to: {}",
                        &chat.id, chat_name, new_chat_name
                    );
                    Ok(())
                }
                Err(err) => {
                    warn!("Chat {} update name error: {}", &chat.id, err.to_string());
                    Err(UserServiceError::ChatNameUpdate(&chat.id))
                }
            }
        } else {
            Ok(())
        }
    } else {
        match ChatDB::create_chat(&pool, &chat.id, &new_chat_name).await {
            Ok(_) => {
                info!("Chat {} created with name: {}", &chat.id, new_chat_name);
                Ok(())
            }
            Err(err) => {
                warn!("Chat {} creating error: {}", &chat.id, err.to_string());
                Err(UserServiceError::ChatCreate(&chat.id))
            }
        }
    }
}

pub async fn process_user<'a>(
    pool: &Pool<Postgres>,
    user: &'a UserRequest,
) -> Result<(), UserServiceError<'a>> {
    if user.is_bot {
        return Err(UserServiceError::UserBotUnable(&user.id));
    };
    let (username, first_name, last_name) = (
        user.username.as_deref().unwrap_or_default(),
        user.first_name.as_deref().unwrap_or_default(),
        user.last_name.as_deref().unwrap_or_default(),
    );
    if let Some(member) = MemberDB::one_by_member_id(pool, &user.id).await {
        if member.username != username
            || member.first_name != first_name
            || member.last_name != last_name
        {
            match MemberDB::update_names(&pool, user.id, username, first_name, last_name).await {
                Ok(_) => {
                    info!("Member {} was updated", user.id);
                    Ok(())
                }
                Err(err) => {
                    warn!("Member {} update error: {}", user.id, err.to_string());
                    Err(UserServiceError::UserUpdate(&user.id))
                }
            }
        } else {
            Ok(())
        }
    } else {
        match MemberDB::create_member(&pool, &user.id, username, first_name, last_name).await {
            Ok(_) => {
                info!("Member {} was created", user.id);
                Ok(())
            }
            Err(err) => {
                warn!("Member {} create error: {}", user.id, err.to_string());
                Err(UserServiceError::UserCreate(&user.id))
            }
        }
    }
}
