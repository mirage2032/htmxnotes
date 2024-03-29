use crate::db::users::User;
use crate::db::DBPool;
use crate::routes::auth::AuthTemplate;
use actix_htmx::Htmx;
use actix_web::{cookie, get, post, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[get("/login")]
pub async fn login_get(_: HttpRequest) -> impl Responder {
    let mut page = AuthTemplate::new("/auth/login", "Login");
    page.add_element("Username", "text", Some("username"));
    page.add_element("Password", "password", Some("password"));
    HttpResponse::Ok().body(page.render().unwrap())
}

#[post("/login")]
pub async fn login_post(
    dbpool: web::Data<DBPool>,
    htmx: Htmx,
    form: web::Form<LoginForm>,
) -> impl Responder {
    let conn = dbpool.as_ref();
    match User::authenticate(conn, &form.username, &form.password) {
        Ok(user) => match user.create_session(conn, Duration::from_secs(60 * 60 * 24 * 2)) {
            Ok(token) => {
                let new_cookie = cookie::Cookie::build("SESSION", token)
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .same_site(cookie::SameSite::Strict)
                    .finish();
                htmx.redirect("/".to_string());
                HttpResponse::Ok().cookie(new_cookie).body("Logged in!")
            }
            Err(err) => HttpResponse::InternalServerError()
                .body(format!("Could not create session: {}", err)),
        },
        Err(_) => HttpResponse::Unauthorized().body("Wrong username or password"),
    }
}
