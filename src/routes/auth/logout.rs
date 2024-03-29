use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use crate::db::DBPool;
use crate::db::sessions::Session;
use crate::middleware::auth::AuthToken;

#[get("/logout")]
pub async fn logout_get(dbpool: web::Data<DBPool>, req: HttpRequest) -> impl Responder {
    // delete token from db if it's found
    let conn = dbpool.as_ref();
    if let Some(token) = req.extensions().get::<AuthToken>(){
        let _ = Session::del_token(conn, &token.value);
    }
    let expired_cookie = Cookie::build("SESSION", "")
    .path("/")
    .expires(time::OffsetDateTime::now_utc() - time::Duration::days(1)).finish();
    HttpResponse::SeeOther()
        .cookie(expired_cookie)
        .insert_header((header::LOCATION, "/"))
        .finish()
}