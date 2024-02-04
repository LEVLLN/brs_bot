use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, FromRow, Pool, Postgres, Row};

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct Member {
    pub id: i32,
    pub member_id: i64,
    pub is_bot: bool,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
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

    pub async fn update_names(
        pool: &Pool<Postgres>,
        member_id: i64,
        username: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<PgQueryResult, Error> {
        query("UPDATE members set username = $1, first_name = $2, last_name = $3, updated_at = $4 where member_id = $5")
            .bind(username)
            .bind(first_name)
            .bind(last_name)
            .bind(Utc::now())
            .bind(member_id)
            .execute(pool)
            .await
    }

    pub async fn create_member(
        pool: &Pool<Postgres>,
        member_id: &i64,
        username: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<PgQueryResult, Error> {
        let now = Utc::now();
        query("INSERT INTO members (is_active, member_id, username, first_name, last_name, is_bot, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(true)
            .bind(member_id)
            .bind(username)
            .bind(first_name)
            .bind(last_name)
            .bind(false)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await
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
    pub async fn create_chat(
        pool: &Pool<Postgres>,
        chat_id: &i64,
        name: &str,
    ) -> Result<PgQueryResult, Error> {
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
