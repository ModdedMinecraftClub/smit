use crate::accounts::api::get_account_or_401;
use crate::api::issues::new_issue;
use crate::db::issues::{fetch_comments_on_issue, fetch_issue};
use crate::response_error::GenericResponse;
use crate::text_check::parse_uuid_or_bad_request;
use actix_session::Session;
use actix_web::web::{scope, Data, Json, Path, ServiceConfig};
use actix_web::HttpResponse;
use actix_web::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

pub fn configure_issues(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/issues")
            .service(new_issue_route)
            .service(get_comments_route)
            .service(get_issue_route),
    );
}

#[derive(Deserialize)]
struct NewIssueDto {
    title: String,
    markdown: String,
}

#[derive(Serialize)]
struct NewIssueResponseDto {
    uuid: Uuid,
}

#[post("")]
async fn new_issue_route(
    pool: Data<MySqlPool>,
    session: Session,
    dto: Json<NewIssueDto>,
) -> GenericResponse {
    let user = get_account_or_401(&session)?;
    let uuid = new_issue(pool.as_ref(), &user, &dto.title, &dto.markdown).await?;
    Ok(HttpResponse::Ok()
        .json(NewIssueResponseDto { uuid })
        .await?)
}

#[get("/{issue_uuid}/comments")]
async fn get_comments_route(
    pool: Data<MySqlPool>,
    Path(issue_uuid): Path<(String)>,
) -> GenericResponse {
    Ok(HttpResponse::Ok().json(
        fetch_comments_on_issue(pool.as_ref(), &parse_uuid_or_bad_request(&issue_uuid)?).await?,
    ))
}

#[get("/{issue_uuid}")]
async fn get_issue_route(
    pool: Data<MySqlPool>,
    Path(issue_uuid): Path<(String)>,
) -> GenericResponse {
    Ok(HttpResponse::Ok()
        .json(fetch_issue(pool.as_ref(), &parse_uuid_or_bad_request(&issue_uuid)?).await?))
}
