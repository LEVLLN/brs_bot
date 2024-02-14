use log::{info, warn};
use sqlx::{Pool, Postgres};
use tokio::try_join;

use crate::common::db::{Chat as ChatDB, ChatId, Member as MemberDB, MemberId};
use crate::common::error::ProcessError;
use crate::common::request::{Chat as ChatRequest, User as UserRequest};

fn chat_title(chat: &ChatRequest) -> String {
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

async fn process_chat<'a>(
    pool: &Pool<Postgres>,
    chat: &ChatRequest,
) -> Result<ChatId, ProcessError<'a>> {
    let new_chat_name = chat_title(chat);
    if let Some((db_id, chat_name)) = ChatDB::id_and_name(pool, chat.id).await {
        if new_chat_name != chat_name {
            match ChatDB::update_name(pool, chat.id, &new_chat_name).await {
                Ok(chat_id) => {
                    info!(
                        "Chat {} name updated from: {} to: {}. Record: {:?}",
                        &chat.id, chat_name, new_chat_name, db_id
                    );
                    Ok(chat_id)
                }
                Err(err) => {
                    warn!("Chat {} update name error: {}", &chat.id, err.to_string());
                    Err(ProcessError::Stop)
                }
            }
        } else {
            Ok(db_id)
        }
    } else {
        match ChatDB::create_chat(pool, chat.id, &new_chat_name).await {
            Ok(chat_id) => {
                info!(
                    "Chat {} created with name: {}. Record: {:?}",
                    &chat.id, new_chat_name, chat_id
                );
                Ok(chat_id)
            }
            Err(err) => {
                warn!("Chat {} creating error: {}", &chat.id, err.to_string());
                Err(ProcessError::Stop)
            }
        }
    }
}

async fn process_user<'a>(
    pool: &Pool<Postgres>,
    user: &UserRequest,
) -> Result<MemberId, ProcessError<'a>> {
    if user.is_bot {
        return Err(ProcessError::Stop);
    }
    let (username, first_name, last_name) = (
        user.username.as_deref().unwrap_or_default(),
        user.first_name.as_deref().unwrap_or_default(),
        user.last_name.as_deref().unwrap_or_default(),
    );
    if let Some(member) = MemberDB::one_by_member_id(pool, user.id).await {
        if member.username != username
            || member.first_name != first_name
            || member.last_name != last_name
        {
            match MemberDB::update_names(pool, user.id, username, first_name, last_name).await {
                Ok(member_id) => {
                    info!(
                        "Member id: {} was updated. Record: {:?}",
                        user.id, member_id
                    );
                    Ok(member_id)
                }
                Err(err) => {
                    warn!("Member id: {} update error: {}", user.id, err.to_string());
                    Err(ProcessError::Stop)
                }
            }
        } else {
            Ok(member.id)
        }
    } else {
        match MemberDB::create_member(pool, user.id, username, first_name, last_name).await {
            Ok(member_id) => {
                info!(
                    "Member id: {} was created to record: {:?}",
                    user.id, member_id
                );
                Ok(member_id)
            }
            Err(err) => {
                warn!("Member {} create error: {}", user.id, err.to_string());
                Err(ProcessError::Stop)
            }
        }
    }
}

async fn bind_user_to_chat<'a>(
    pool: &Pool<Postgres>,
    member_id: MemberId,
    chat_id: ChatId,
) -> Result<(MemberId, ChatId), ProcessError<'a>> {
    if MemberDB::is_in_chat(pool, &member_id, &chat_id).await {
        Ok((member_id, chat_id))
    } else {
        match MemberDB::bind_to_chat(pool, &member_id, &chat_id).await {
            Ok(chat_to_member_id) => {
                info!(
                    "Member {:?} binds to Chat {:?} success. Record: {:?}",
                    &member_id, &chat_id, chat_to_member_id
                );
                Ok((member_id, chat_id))
            }
            Err(err) => {
                warn!(
                    "Member {:?} can't binds to Chat {:?}. Error: {}",
                    member_id,
                    chat_id,
                    err.to_string()
                );
                Err(ProcessError::Stop)
            }
        }
    }
}

pub async fn process_user_and_chat<'a>(
    pool: &Pool<Postgres>,
    user: &UserRequest,
    chat: &ChatRequest,
) -> Result<(MemberId, ChatId), ProcessError<'a>> {
    match try_join!(process_user(pool, user), process_chat(pool, chat)) {
        Ok((member_id, chat_id)) => bind_user_to_chat(pool, member_id, chat_id).await,
        Err(e) => Err(e),
    }
}
