use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use actix_web::cookie::Cookie;
use askama::Template;
use serde::{Deserialize, Serialize};
use crate::routes::auth::AuthTemplate;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[get("/register")]
pub async fn register_get(_: HttpRequest) -> impl Responder {
    let mut page = AuthTemplate::new("/auth/register", "Register");
    page.add_element("Username", "text", Some("username"));
    page.add_element("Email", "email", Some("email"));
    page.add_element("Password", "password", Some("password"));
    page.add_element("Verify password:", "password", None);
    HttpResponse::Ok().body(page.render().unwrap())
}

#[post("/register")]
pub async fn register_post(form: web::Form<RegisterForm>) -> impl Responder {
    println!("Username: {:?}", form);
    HttpResponse::Ok()
        .cookie(Cookie::build("SESSION", form.username.clone()).path("/").finish())
        .insert_header(("HX-Redirect", "/"))
        .body("Success!")
}
