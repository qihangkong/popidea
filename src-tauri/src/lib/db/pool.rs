use sqlx::SqlitePool;

pub struct DatabasePool {
    pool: SqlitePool,
}

impl DatabasePool {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn inner(&self) -> &SqlitePool {
        &self.pool
    }
}
