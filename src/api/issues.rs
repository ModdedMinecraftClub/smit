use crate::accounts::api::AuthenticatedUser;
use crate::response_error::{GenericError, GenericResult};
use crate::text_check::validate_and_sanitize_string;
use crate::util::current_unix_time_secs;
use sqlx::MySqlPool;
use uuid::Uuid;

mod db {
    pub use crate::db::issues::*;
}

pub async fn new_issue(
    pool: &MySqlPool,
    user: &AuthenticatedUser,
    title: &str,
    markdown: &str,
) -> GenericResult<Uuid> {
    //validate data
    let title = validate_and_sanitize_string(title, false)?;
    let markdown = validate_and_sanitize_string(markdown, true)?;

    if title.len() < 2 || title.len() >= 128 {
        return Err(GenericError::BrokenInvariant(
            "The title must be between 2 and 128 characters.".into(),
        ));
    }

    if markdown.len() < 2 {
        return Err(GenericError::BrokenInvariant(
            "The markdown must be at leats 2 characters.".into(),
        ));
    }

    //do the thing
    let issue_uuid = db::new_issue(pool, &user.email, &title).await?;
    let _ = db::new_comment(
        pool,
        &issue_uuid,
        &user.email,
        &markdown,
        current_unix_time_secs() as i64,
    )
    .await?;

    Ok(issue_uuid)
}
