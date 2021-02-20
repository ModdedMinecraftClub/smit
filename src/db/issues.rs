use crate::response_error::GenericResult;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn new_issue(pool: &MySqlPool, author_email: &str, title: &str) -> GenericResult<Uuid> {
    //generate primary key
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();

    //insert
    sqlx::query!(
        "INSERT INTO issues(uuid, author, title) VALUES(?, ?, ?)",
        uuid_str,
        author_email,
        title
    )
    .execute(pool)
    .await?;

    Ok(uuid)
}

pub async fn new_comment(
    pool: &MySqlPool,
    issue: &Uuid,
    author_email: &str,
    markdown: &str,
    timestamp: i64,
) -> GenericResult<Uuid> {
    //generate primary key
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();

    //insert
    sqlx::query!(
        "INSERT INTO comments(uuid, issue_uuid, author, contents, timestamp) VALUES(?, ?, ?, ?, ?)",
        uuid_str,
        issue.to_string(),
        author_email,
        markdown,
        timestamp
    )
    .execute(pool)
    .await?;

    Ok(uuid)
}
