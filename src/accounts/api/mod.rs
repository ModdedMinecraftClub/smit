use crate::accounts::api::cryptography::{hash_password, verify_hash};
use crate::accounts::api::db::{fetch_user, insert_user_unvalidated, User};
use crate::response_error::{GenericError, GenericResult};
use crate::text_check::{validate_and_sanitize_string, validate_email};
use actix_session::Session;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

mod cryptography;
mod db;

#[derive(Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub email: String,
    pub username: String,
}

pub async fn sign_up_user(
    pool: &MySqlPool,
    email: &str,
    username: &str,
    password: &str,
) -> GenericResult<()> {
    //validate input
    let username = validate_and_sanitize_string(username, false)?;
    let email = validate_and_sanitize_string(email, false)?;
    let _ = validate_email(&email)?;

    if username.len() < 2 || username.len() > 64 {
        return Err(GenericError::MalformedRequest(
            "Your username must be between 2 and 64 characters in length.".into(),
        ));
    }

    //verify that the email isn't taken
    if let Some(_) = fetch_user(pool, &email).await? {
        return Err(GenericError::Conflict(
            "Your selected email is already taken.".into(),
        ));
    }

    //password hash
    let hash_data = hash_password(password)?; //no verification is needed on the server side since it's the client's loss if the password doesn't meet the requirements

    //registration
    insert_user_unvalidated(
        pool,
        &User {
            username,
            email,
            password_hash: hash_data.hash.into(),
            password_salt: hash_data.salt.into(),
        },
    )
    .await?;

    //TODO: Email Verification

    Ok(())
}

pub async fn sign_in_user(
    pool: &MySqlPool,
    session: &Session,
    email: &str,
    password: &str,
) -> GenericResult<()> {
    //verify credentials
    let user = match fetch_user(pool, email).await? {
        Some(user) => user,
        None => {
            return Err(GenericError::Custom(
                "Email or password is incorrect".into(),
                StatusCode::UNAUTHORIZED,
            ))
        }
    };

    if !verify_hash(&user.password_hash, &user.password_salt, password)? {
        return Err(GenericError::Custom(
            "Email or password is incorrect".into(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    //sign in
    session.set(
        "account",
        AuthenticatedUser {
            email: user.email,
            username: user.username,
        },
    )?;

    Ok(())
}

pub fn sign_out_user(session: &Session) -> GenericResult<()> {
    session.remove("account");
    Ok(())
}

pub fn get_account_or_401(session: &Session) -> GenericResult<AuthenticatedUser> {
    match session.get("account")? {
        Some(account) => Ok(account),
        None => Err(GenericError::Custom(
            "Not Authorized".into(),
            StatusCode::UNAUTHORIZED,
        )),
    }
}
