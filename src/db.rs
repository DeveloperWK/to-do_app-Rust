use sqlx::{Pool, Postgres};

pub type DBpool = Pool<Postgres>;

pub async fn connect_db() -> DBpool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::<Postgres>::connect(&url)
        .await
        .expect("Could not connect to database")
}
