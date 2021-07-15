use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqlitePool};

// this struct will be used to represent database record
#[derive(Serialize, FromRow, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
}

// implementation of Actix Responder for User struct so we can return User from action handler
impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Implementation for User struct, functions for read/write/update and delete User from database
impl User {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<User>> {
        let mut todos = vec![];
        let recs = sqlx::query!("SELECT id,name,phone,email,password FROM users ORDER BY id")
            .fetch_all(pool)
            .await?;

        for rec in recs {
            todos.push(User {
                id: i64::from(rec.id),
                name: rec.name,
                email: rec.email,
                phone: rec.phone,
                password: rec.password,
            });
        }
        Ok(todos)
    }

    pub async fn find_by_id(id: i64, pool: &SqlitePool) -> Result<User> {
        let rec = sqlx::query!("SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;

        Ok(User {
            id: i64::from(rec.id),
            name: rec.name,
            email: rec.email,
            phone: rec.phone,
            password: rec.password,
        })
    }

    pub async fn create(user: User, pool: &SqlitePool) -> Result<User> {
        let mut tx = pool.begin().await?;
        let u = sqlx::query(
            "INSERT INTO users (name,email,phone) VALUES ($1, $2,$3) RETURNING id, name,email,phone",
        )
        .bind(&user.name)
        .bind(user.email)
        .bind(user.phone)
        .map(|row: SqliteRow| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            phone: row.get(3),
            password: "".to_string(),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(u)
    }

    pub async fn update(id: i64, user: User, pool: &SqlitePool) -> Result<User> {
        let mut tx = pool.begin().await.unwrap();
        let user = sqlx::query(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email,phone",
        )
        .bind(&user.name)
        .bind(user.email)
        .bind(id)
        .map(|row: SqliteRow| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            phone: row.get(3),
            password: "".to_string(),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await.unwrap();
        Ok(user)
    }

    pub async fn delete(id: i64, pool: &SqlitePool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted)
    }
}
