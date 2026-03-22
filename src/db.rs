use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(db: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db_options = SqliteConnectOptions::from_str(db)?
            .create_if_missing(true)
            .disable_statement_logging()
            .to_owned();

        let pool = SqlitePoolOptions::new().connect_with(db_options).await?;

        sqlx::query(
            "
CREATE TABLE IF NOT EXISTS User(
	user_id INTEGER PRIMARY KEY,
	username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
	password_hash BINARY(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS Post(
	post_id INTEGER PRIMARY KEY,
	user_id INTEGER NOT NULL,
	title VARCHAR(60) NOT NULL,
	slug VARCHAR(255) NOT NULL UNIQUE,
	contents TEXT NOT NULL,
	FOREIGN KEY(user_id) REFERENCES User(user_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_post_id ON Post(post_id);
CREATE INDEX IF NOT EXISTS idx_post_slug on Post(slug);
CREATE INDEX IF NOT EXISTS idx_post_user on Post(user_id);
CREATE INDEX IF NOT EXISTS idx_user_id ON User(user_id);
CREATE INDEX IF NOT EXISTS idx_user_name ON User(username);
        ",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}
