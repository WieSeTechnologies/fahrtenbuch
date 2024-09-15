use sqlx::PgPool;
use sqlx::Row;
use tracing::debug;

pub async fn fetch_user_count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let user_count = sqlx::query("SELECT count(*) FROM users")
        .fetch_one(pool)
        .await?;

    debug!("User Count: {:?}", &user_count);

    user_count.try_get("count")
}
