use actix_web::{get, HttpRequest, HttpResponse, post, Responder, cookie::Cookie, web};
use askama::Template;
use serde::{Deserialize, Serialize};
use crate::routes::auth::AuthTemplate;

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
pub async fn login_post(form: web::Form<LoginForm>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(Cookie::build("SESSION", form.username.clone()).path("/").finish())
        .insert_header(("HX-Redirect", "/"))
        .body("Success!")
}
