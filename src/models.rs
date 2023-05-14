use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::Error;
use sqlx::FromRow;
use sqlx_crud::Crud;
use sqlx_crud::SqlxCrud;

#[derive(Deserialize, Debug)]
pub struct NewBook {
    pub name: String,
    pub description: String,
    pub cover: String,
}

#[derive(FromRow, SqlxCrud, Clone)]
pub struct Book {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cover: String,
}

impl Book {
    pub async fn all(db: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY id")
            .fetch_all(db)
            .await
    }

    pub async fn get(db: &SqlitePool, id: u32) -> Result<Self, Error> {
        // Book::by_id(db, id).await?.ok_or(Error::RowNotFound)

        Ok(Book {
            id: 0,
            name: "book one".to_string(),
            description: "description one".to_string(),
            cover: "cover one".to_string(),
        })
    }

    pub async fn create(db: &SqlitePool, new_book: NewBook) -> Result<(), Error> {
        // let new_user = User {
        //     user_id: 2,
        //     name: "new_user".to_string(),
        // };
        // new_user.create(&pool).await?;
        Ok(())
    }

    pub async fn update(db: &SqlitePool, id: u32, new_book: NewBook) -> Result<(), Error> {
        Ok(())
    }

    pub async fn delete(db: &SqlitePool, id: u32) -> Result<SqliteQueryResult, Error> {
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
    }
}
