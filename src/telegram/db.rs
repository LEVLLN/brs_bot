use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres, query, query_as, Row};
use sqlx::postgres::PgQueryResult;

#[derive(Debug, FromRow)]
pub struct Member {
    id: i32,
    member_id: i64,
    is_bot: bool,
    username: String,
    first_name: String,
    last_name: String,
}

impl Member {
    pub async fn all(&self, pool: Pool<Postgres>) -> Vec<Member> {
        query_as::<_, Member>(
            "SELECT id, member_id, is_bot, username, last_name, first_name from members;",
        )
        .fetch_all(&pool)
        .await
        .ok()
        .unwrap()
    }
    pub async fn one_by_member_id(pool: &Pool<Postgres>, member_id: &i64) -> Option<Member> {
        query_as::<_, Member>(
            &format!("SELECT id, member_id, is_bot, username, last_name, first_name from members where member_id = {member_id};"),
        )
            .fetch_one(pool)
            .await
            .ok()
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct Chat {
    id: i32,
    chat_id: i64,
    name: String,
}

impl Chat {
    pub async fn all(pool: &Pool<Postgres>) -> Vec<Chat> {
        query_as::<_, Chat>("SELECT id, chat_id, name from chats where chat_id > 0;")
            .fetch_all(pool)
            .await
            .ok()
            .unwrap()
    }
    pub async fn chat_name(pool: &Pool<Postgres>, chat_id: &i64) -> Option<String> {
        query("SELECT name FROM chats WHERE chat_id = $1")
            .bind(chat_id)
            .fetch_one(pool)
            .await
            .ok()
            .map(|x| x.get::<String, _>("name"))
    }
    pub async fn create_chat(pool: &Pool<Postgres>, chat_id: &i64, name: &str) -> Result<PgQueryResult, Error> {
        let now = Utc::now();
        query("INSERT INTO chats (is_active, chat_id, name, morph_answer_chance, is_openai_enabled, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7)")
            .bind(true)
            .bind(chat_id)
            .bind(name)
            .bind(15)
            .bind(false)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await
    }
    pub async fn update_name(
        pool: &Pool<Postgres>,
        chat_id: &i64,
        name: &str,
    ) -> Result<PgQueryResult, Error> {
        query("UPDATE chats set name = $1, updated_at = $2 where chat_id = $3")
            .bind(name)
            .bind(Utc::now())
            .bind(chat_id)
            .execute(pool)
            .await
    }
}
