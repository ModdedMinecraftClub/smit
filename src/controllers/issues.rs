use crate::accounts::api::get_account_or_401;
use crate::api::issues::new_issue;
use crate::response_error::GenericResponse;
use actix_session::Session;
use actix_web::post;
use actix_web::web::{scope, Data, Json, ServiceConfig};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

pub fn configure_issues(cfg: &mut ServiceConfig) {
    cfg.service(scope("/issues").service(new_issue_route));
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

#[post("new")]
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
