use serde::Deserialize;
use sqlx::{
    query, query_as,
    sqlite::{SqlitePool, SqliteQueryResult},
    Error, FromRow,
};

#[derive(Deserialize, Debug)]
pub struct NewBook {
    pub name: String,
    pub description: String,
    pub cover: String,
}

#[derive(FromRow, Clone)]
pub struct Book {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cover: String,
}

impl Book {
    pub async fn list(db: &SqlitePool) -> Result<Vec<Self>, Error> {
        query_as::<_, Book>("SELECT * FROM books ORDER BY id")
            .fetch_all(db)
            .await
    }

    pub async fn fetch(db: &SqlitePool, id: u32) -> Result<Self, Error> {
        query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
    }

    pub async fn create(db: &SqlitePool, new_book: NewBook) -> Result<SqliteQueryResult, Error> {
        query("INSERT into books (name, description, cover) values (?, ?, ?)")
            .bind(new_book.name)
            .bind(new_book.description)
            .bind(new_book.cover)
            .execute(db)
            .await
    }

    pub async fn update(
        db: &SqlitePool,
        id: u32,
        updated_book: NewBook,
    ) -> Result<SqliteQueryResult, Error> {
        query("UPDATE books SET name = ?, description = ?, cover = ? WHERE id = ?")
            .bind(updated_book.name)
            .bind(updated_book.description)
            .bind(updated_book.cover)
            .bind(id)
            .execute(db)
            .await
    }

    pub async fn delete(db: &SqlitePool, id: u32) -> Result<SqliteQueryResult, Error> {
        query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
    }
}
