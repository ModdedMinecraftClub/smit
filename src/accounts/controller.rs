use crate::accounts::api::{get_account_or_401, sign_in_user, sign_out_user, sign_up_user};
use crate::response_error::GenericResult;
use actix_session::Session;
use actix_web::web::{scope, Data, Json, ServiceConfig};
use actix_web::{get, post, HttpResponse};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn configure_accounts(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/accounts")
            .service(status)
            .service(signin)
            .service(signout)
            .service(signup),
    );
}

#[derive(Deserialize)]
struct SignupDto {
    email: String,
    username: String,
    password: String,
}

#[post("/signup")]
async fn signup(pool: Data<MySqlPool>, dto: Json<SignupDto>) -> GenericResult<HttpResponse> {
    sign_up_user(pool.as_ref(), &dto.email, &dto.username, &dto.password).await?;
    Ok(HttpResponse::Ok().await?)
}

#[derive(Deserialize)]
struct SigninDto {
    email: String,
    password: String,
}

#[post("/signin")]
async fn signin(
    pool: Data<MySqlPool>,
    session: Session,
    dto: Json<SigninDto>,
) -> GenericResult<HttpResponse> {
    sign_in_user(pool.as_ref(), &session, &dto.email, &dto.password).await?;
    Ok(HttpResponse::Ok().await?)
}

#[get("/signout")] //TODO: Fix this when upstream fix is done, needs to be get due to https://github.com/actix/actix-web/issues/2007
async fn signout(session: Session) -> GenericResult<HttpResponse> {
    sign_out_user(&session)?;
    Ok(HttpResponse::Ok().await?)
}

#[get("/status")]
async fn status(session: Session) -> GenericResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(get_account_or_401(&session)?))
}
