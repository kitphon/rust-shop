use sea_orm::{Database, DatabaseConnection};

pub async fn connect_to_db(database_url: &str) -> DatabaseConnection {
    Database::connect(database_url)
        .await
        .expect("Failed to connect to the database")
}
