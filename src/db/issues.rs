use crate::response_error::{GenericError, GenericResponse, GenericResult};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::str::FromStr;
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

#[derive(Serialize)]
pub struct Issue {
    pub uuid: Uuid,
    pub username: String,
    pub title: String,
}

pub async fn fetch_issue(pool: &MySqlPool, uuid: &Uuid) -> GenericResult<Issue> {
    let issue = sqlx::query!(
        r#"SELECT username, title FROM issues INNER JOIN users ON issues.author=users.email WHERE uuid=?"#,
        uuid.to_string()
    )
    .fetch_one(pool)
    .await?;
    Ok(Issue {
        uuid: *uuid,
        username: issue.username,
        title: issue.title,
    })
}

#[derive(Serialize)]
pub struct Comment {
    pub uuid: Uuid,
    pub username: String,
    pub contents: String,
    pub timestamp: i64,
}

pub async fn fetch_comments_on_issue(
    pool: &MySqlPool,
    issue_uuid: &Uuid,
) -> GenericResult<Vec<Comment>> {
    Ok(sqlx::query!(
        "SELECT comments.uuid, username, contents, timestamp FROM comments INNER JOIN users ON comments.author=users.email WHERE issue_uuid=?",
        issue_uuid.to_string()
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| {
        Ok(Comment {
            uuid: Uuid::from_str(&record.uuid)?,
            username: record.username,
            contents: record.contents,
            timestamp: record.timestamp,
        })
    })
    .collect::<GenericResult<Vec<Comment>>>()?)
}
