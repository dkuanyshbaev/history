use serde::Deserialize;
use sqlx::{
    query, query_as,
    sqlite::{SqlitePool, SqliteQueryResult},
    Error, FromRow,
};

#[derive(Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub lead: String,
    pub body: String,
    pub cover: String,
}

#[derive(FromRow, Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub lead: String,
    pub body: String,
    pub cover: String,
}

impl Post {
    pub async fn list(db: &SqlitePool) -> Result<Vec<Self>, Error> {
        query_as::<_, Post>("SELECT * FROM posts ORDER BY id")
            .fetch_all(db)
            .await
    }

    pub async fn fetch(db: &SqlitePool, id: u32) -> Result<Self, Error> {
        query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
    }

    pub async fn create(db: &SqlitePool, new_post: NewPost) -> Result<SqliteQueryResult, Error> {
        query("INSERT into posts (title, lead, body, cover) values (?, ?, ?, ?)")
            .bind(new_post.title)
            .bind(new_post.lead)
            .bind(new_post.body)
            .bind(new_post.cover)
            .execute(db)
            .await
    }

    pub async fn update(
        db: &SqlitePool,
        id: u32,
        updated_post: NewPost,
    ) -> Result<SqliteQueryResult, Error> {
        query("UPDATE posts SET title = ?, lead = ?, body = ?, cover = ? WHERE id = ?")
            .bind(updated_post.title)
            .bind(updated_post.lead)
            .bind(updated_post.body)
            .bind(updated_post.cover)
            .bind(id)
            .execute(db)
            .await
    }

    pub async fn delete(db: &SqlitePool, id: u32) -> Result<SqliteQueryResult, Error> {
        query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
    }
}
