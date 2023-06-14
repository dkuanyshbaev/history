use serde::Deserialize;
use sqlx::{
    query, query_as,
    sqlite::{SqlitePool, SqliteQueryResult},
    Error, FromRow,
};

#[derive(Deserialize, Debug)]
pub struct NewTextBook {
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: String,
}

#[derive(FromRow, Clone)]
pub struct TextBook {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub description: String,
    pub cover: String,
}

impl TextBook {
    pub async fn list(db: &SqlitePool) -> Result<Vec<Self>, Error> {
        query_as::<_, TextBook>("SELECT * FROM textbooks ORDER BY id")
            .fetch_all(db)
            .await
    }

    pub async fn fetch(db: &SqlitePool, id: u32) -> Result<Self, Error> {
        query_as::<_, TextBook>("SELECT * FROM textbooks WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
    }

    pub async fn create(
        db: &SqlitePool,
        new_textbook: NewTextBook,
    ) -> Result<SqliteQueryResult, Error> {
        query("INSERT into textbooks (name, link, description, cover) values (?, ?, ?, ?)")
            .bind(new_textbook.name)
            .bind(new_textbook.link)
            .bind(new_textbook.description)
            .bind(new_textbook.cover)
            .execute(db)
            .await
    }

    pub async fn update(
        db: &SqlitePool,
        id: u32,
        updated_textbook: NewTextBook,
    ) -> Result<SqliteQueryResult, Error> {
        query("UPDATE textbooks SET name = ?, link = ?, description = ?, cover = ? WHERE id = ?")
            .bind(updated_textbook.name)
            .bind(updated_textbook.link)
            .bind(updated_textbook.description)
            .bind(updated_textbook.cover)
            .bind(id)
            .execute(db)
            .await
    }

    pub async fn delete(db: &SqlitePool, id: u32) -> Result<SqliteQueryResult, Error> {
        query("DELETE FROM textbooks WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
    }
}
