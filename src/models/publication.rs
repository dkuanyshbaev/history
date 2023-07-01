use serde::Deserialize;
use sqlx::{
    query, query_as,
    sqlite::{SqlitePool, SqliteQueryResult},
    Error, FromRow,
};

#[derive(Deserialize, Debug)]
pub struct NewPublication {
    pub name: String,
    pub link: String,
    pub description: String,
}

#[derive(FromRow, Clone)]
pub struct Publication {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub description: String,
}

impl Publication {
    pub async fn list(db: &SqlitePool) -> Result<Vec<Self>, Error> {
        query_as::<_, Publication>("SELECT * FROM publications ORDER BY id")
            .fetch_all(db)
            .await
    }

    pub async fn fetch(db: &SqlitePool, id: u32) -> Result<Self, Error> {
        query_as::<_, Publication>("SELECT * FROM publications WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
    }

    pub async fn create(
        db: &SqlitePool,
        new_publication: NewPublication,
    ) -> Result<SqliteQueryResult, Error> {
        query("INSERT into publications (name, link, description) values (?, ?, ?)")
            .bind(new_publication.name)
            .bind(new_publication.link)
            .bind(new_publication.description)
            .execute(db)
            .await
    }

    pub async fn update(
        db: &SqlitePool,
        id: u32,
        updated_publication: NewPublication,
    ) -> Result<SqliteQueryResult, Error> {
        query("UPDATE publications SET name = ?, link = ?, description = ? WHERE id = ?")
            .bind(updated_publication.name)
            .bind(updated_publication.link)
            .bind(updated_publication.description)
            .bind(id)
            .execute(db)
            .await
    }

    pub async fn delete(db: &SqlitePool, id: u32) -> Result<SqliteQueryResult, Error> {
        query("DELETE FROM publications WHERE id = ?")
            .bind(id)
            .execute(db)
            .await
    }
}
