use actix_web::{get, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::http::header;

#[get("/logout")]
pub async fn logout_get(_: HttpRequest) -> impl Responder {
    HttpResponse::SeeOther()
        .cookie(Cookie::build("SESSION", "").path("/").expires(time::OffsetDateTime::now_utc() - time::Duration::days(1)).finish())
        .insert_header((header::LOCATION, "/"))
        .finish()
}