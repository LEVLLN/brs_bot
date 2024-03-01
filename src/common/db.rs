use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error, FromRow, PgPool, Pool, Postgres, Row};

#[derive(Clone, Serialize, Deserialize, Debug, FromRow, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct ChatToMemberId(i32);

#[derive(Clone, Serialize, Deserialize, Debug, FromRow, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct MemberId(i32);

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct Member {
    pub id: MemberId,
    pub member_id: i64,
    pub is_bot: bool,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, FromRow, sqlx::Type, PartialEq)]
#[sqlx(transparent)]
pub struct ChatId(i32);

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct Chat {
    pub id: ChatId,
    pub chat_id: i64,
    pub name: String,
}

impl ChatId {
    #[allow(dead_code)]
    pub fn new(v: i32) -> Self {
        ChatId(v)
    }
}

impl MemberId {
    #[allow(dead_code)]
    pub fn new(v: i32) -> Self {
        MemberId(v)
    }
}

impl Member {
    pub async fn one_by_id(pool: &PgPool, id: &MemberId) -> Option<Member> {
        query_as::<_, Member>(
            "SELECT id, member_id, is_bot, username, last_name, first_name FROM members WHERE id = $1;",
        ).bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }
    pub async fn one_by_member_id(pool: &Pool<Postgres>, member_id: i64) -> Option<Member> {
        query_as::<_, Member>(
            &format!("SELECT id, member_id, is_bot, username, last_name, first_name FROM members WHERE member_id = {member_id};"),
        )
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn update_names(
        pool: &Pool<Postgres>,
        member_id: i64,
        username: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<MemberId, Error> {
        query("UPDATE members SET username = $1, first_name = $2, last_name = $3, updated_at = now() \
        WHERE member_id = $4\
        RETURNING id;")
            .bind(username)
            .bind(first_name)
            .bind(last_name)
            .bind(member_id)
            .fetch_one(pool)
            .await.map(|x| x.get::<MemberId, _>("id"))
    }

    pub async fn create_member(
        pool: &Pool<Postgres>,
        member_id: i64,
        username: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<MemberId, Error> {
        query(
            "INSERT INTO members \
        (is_active, member_id, username, first_name, last_name, is_bot, created_at, updated_at) \
        VALUES (true, $1, $2, $3, $4, false, now(), now())\
        RETURNING id",
        )
        .bind(member_id)
        .bind(username)
        .bind(first_name)
        .bind(last_name)
        .fetch_one(pool)
        .await
        .map(|x| x.get::<MemberId, _>("id"))
    }

    pub async fn update_chat_to_member_bind(
        pool: &Pool<Postgres>,
        member_id: &MemberId,
        chat_id: &ChatId,
    ) -> Option<ChatToMemberId> {
        query(
            "UPDATE chats_to_members SET updated_at=now() \
        WHERE member_id = $1 \
        AND chat_id = $2 \
        RETURNING id;",
        )
        .bind(member_id)
        .bind(chat_id)
        .fetch_one(pool)
        .await
        .ok()
        .map(|x| x.get::<ChatToMemberId, _>("id"))
    }

    pub async fn bind_to_chat(
        pool: &Pool<Postgres>,
        member_id: &MemberId,
        chat_id: &ChatId,
    ) -> Result<ChatToMemberId, Error> {
        query(
            "INSERT INTO chats_to_members (member_id, chat_id, updated_at, created_at, is_active) \
        VALUES ($1, $2, now(), now(), true) \
        RETURNING id;",
        )
        .bind(member_id)
        .bind(chat_id)
        .fetch_one(pool)
        .await
        .map(|x| x.get::<ChatToMemberId, _>("id"))
    }

    pub async fn chat_members(pool: &Pool<Postgres>, chat_id: &ChatId) -> Vec<MemberId> {
        query(
            "SELECT member_id FROM chats_to_members \
        WHERE chat_id = $1 AND updated_at >= now() - INTERVAL '30 DAYS'",
        )
        .bind(chat_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .iter()
        .map(|x| x.get::<MemberId, _>("member_id"))
        .collect::<Vec<MemberId>>()
    }
}

impl Chat {
    pub async fn id_and_name(pool: &Pool<Postgres>, chat_id: i64) -> Option<(ChatId, String)> {
        query("SELECT id, name FROM chats WHERE chat_id = $1")
            .bind(chat_id)
            .fetch_one(pool)
            .await
            .ok()
            .map(|x| (x.get::<ChatId, _>("id"), x.get::<String, _>("name")))
    }
    pub async fn create_chat(
        pool: &Pool<Postgres>,
        chat_id: i64,
        name: &str,
    ) -> Result<ChatId, Error> {
        query(
            "INSERT INTO chats \
        (is_active, chat_id, name, morph_answer_chance, is_openai_enabled, created_at, updated_at) \
        VALUES (true, $1, $2, 15, false, now(), now())\
        RETURNING id;",
        )
        .bind(chat_id)
        .bind(name)
        .fetch_one(pool)
        .await
        .map(|x| x.get::<ChatId, _>("id"))
    }
    pub async fn update_name(
        pool: &Pool<Postgres>,
        chat_id: i64,
        name: &str,
    ) -> Result<ChatId, Error> {
        query("UPDATE chats SET name = $1, updated_at = now() where chat_id = $2 RETURNING id;")
            .bind(name)
            .bind(chat_id)
            .fetch_one(pool)
            .await
            .map(|x| x.get::<ChatId, _>("id"))
    }
}
