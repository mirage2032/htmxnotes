use actix_web::web;

mod login;
mod register;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login_get);
    cfg.service(login::login_post);
    cfg.service(web::resource("/register").get(register::register));
}
