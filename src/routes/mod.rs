use actix_web::web;

mod auth;
mod index;

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index::index)));
    cfg.service(web::scope("/auth").configure(auth::auth_routes));
}
