use serde::Deserialize;
use sqlx::{
    query, query_as,
    sqlite::{SqlitePool, SqliteQueryResult},
    Error, FromRow,
};

#[derive(Deserialize, Debug)]
pub struct NewText {
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: String,
}

#[derive(FromRow, Clone)]
pub struct Text {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: String,
}

impl Text {
    pub async fn list(db: &SqlitePool) -> Result<Vec<Self>, Error> {
        query_as::<_, Text>("SELECT * FROM texts ORDER BY id")
            .fetch_all(db)
            .await
    }

    pub async fn fetch(db: &SqlitePool, id: u32) -> Result<Self, Error> {
        query_as::<_, Text>("SELECT * FROM texts WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
    }

    pub async fn create(db: &SqlitePool, new_text: NewText) -> Result<SqliteQueryResult, Error> {
        query("INSERT into texts (name, link, description, cover) values (?, ?, ?, ?)")
            .bind(new_text.name)
            .bind(new_text.link)
            .bind(new_text.description)
            .bind(new_text.cover)
            .execute(db)
            .await
    }

    pub async fn update(
        db: &SqlitePool,
        id: u32,
        updated_text: NewText,
    ) -> Result<SqliteQueryResult, Error> {
        query("UPDATE texts SET name = ?, link = ?, description = ?, cover = ? WHERE id = ?")
            .bind(updated_text.name)
            .bind(updated_text.link)
            .bind(updated_text.description)
            .bind(updated_text.cover)
            .bind(id)
            .execute(db)
            .await
    }

    pub async fn delete(db: &SqlitePool, id: u32) -> Result<SqliteQueryResult, Error> {
        query("DELETE FROM texts WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
    }
}
