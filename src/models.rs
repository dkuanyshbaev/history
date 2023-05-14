use sqlx::sqlite::SqlitePool;

#[derive(sqlx::FromRow, Clone)]
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
    // pub async fn get(db: &SqlitePool) -> Result<Self, sqlx::Error> {
    //     Ok(())
    // }
    // pub async fn create(db: &SqlitePool) -> Result<(), sqlx::Error> {
    //     Ok(())
    // }
    // pub async fn update(db: &SqlitePool) -> Result<(), sqlx::Error> {
    //     Ok(())
    // }
    // pub async fn delete(db: &SqlitePool) -> Result<(), sqlx::Error> {
    //     Ok(())
    // }
}
