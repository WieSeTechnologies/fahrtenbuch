use crate::data_models::session::Session;
use crate::data_models::user::LoginUser;
use crate::data_models::user::User;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use chrono::Duration;
use chrono::Local;
use regex::Regex;
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

pub async fn login(
    login_user: &LoginUser,
    pool: &PgPool,
) -> Result<Session, Box<dyn std::error::Error>> {
    // Check if the username is valid
    let re = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    if !re.is_match(&login_user.username) {
        return Err("Username contains invalid characters.".into());
    }

    // Check for length
    if login_user.username.len() > 255 {
        return Err("The username is too long.".into());
    }

    // Get the User
    let user_query = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(&login_user.username)
        .fetch_one(pool)
        .await?;

    let user: User = User::try_from(user_query)?;

    // Verify the provided password with the one in the table
    // TODO: Cast the Error into Anyhow?
    let password_hash = match PasswordHash::new(&user.password_hash).unwrap() {
        Ok(hash) => hash,
        Err(e) => {
            return Err("Error whilst getting the password hash.".into());
        }
    };
    let password_check = Argon2::default()
        .verify_password(login_user.password.as_bytes(), &password_hash)
        .is_ok();

    if !password_check {
        debug!("Password check NOT passed.");
        return Err("The provided password does not match.".into());
    } else {
        debug!("Password check passed.")
    }

    // Create A new Session
    let session = Session {
        uuid: Uuid::new_v4(),
        expiry: Local::now() + Duration::days(30),
        owner_username: user.username,
    };

    // Insert the session into the database
    let session_insert_query =
        sqlx::query("INSERT INTO sessions (uuid, expiry, owner_username) VALUES ($1, $2, $3)")
            .bind(&session.uuid)
            .bind(&session.expiry)
            .bind(&session.owner_username)
            .execute(pool)
            .await?;

    if session_insert_query.rows_affected() < 1 {
        return Err("Could not insert session into sessions table.".into());
    }

    Ok(session)
}
