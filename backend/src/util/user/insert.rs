use crate::data_models::user::User;
use crate::util::user::check_username::check_username;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;
use sqlx::Row;
use tracing::debug;

pub async fn insert_user(
    user: &User,
    pool: &PgPool,
) -> Result<PgQueryResult, Box<dyn std::error::Error>> {
    // Check if the username is valid
    if !check_username(&user.username) {
        return Err("Username contains invalid characters.".into());
    }

    // Check for length
    if user.username.len() > 255 {
        return Err("The username is too long.".into());
    } else if user.displayname.len() > 255 {
        return Err("The displayname is too long.".into());
    } else if user.password_hash.len() > 255 {
        return Err("The password hash is too long.".into());
    }

    // Check if the username already exists
    let count_query = sqlx::query("SELECT count(*) FROM users WHERE username = $1")
        .bind(&user.username)
        .fetch_one(pool)
        .await?;

    let count: i64 = count_query.try_get("count")?;
    if count > 0 {
        return Err("Username already exists.".into());
    }

    // Create the user
    let creation_query_result = sqlx::query("INSERT INTO users (username, displayname, password_hash, new_password_required, creation_date, role) VALUES ($1, $2, $3, $4, $5, $6)")
    .bind(&user.username)
    .bind(&user.displayname)
    .bind(&user.password_hash)
    .bind(&user.new_password_required)
    .bind(&user.creation_date)
    .bind(&user.role)
    .execute(pool).await?;

    debug!("Query Result: {:?}", &creation_query_result);

    Ok(creation_query_result)
}
