use actix_files as fs;
use actix_htmx::HtmxMiddleware;
use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;

mod db;
mod middleware;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "127.0.0.1:8000";
    println!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new()
            .wrap(HtmxMiddleware)
            .wrap(from_fn(middleware::auth::authenticate))
            .wrap(from_fn(middleware::nocache::nocache))
            .app_data(web::Data::new(db::create_pool()))
            .configure(routes::all_routes)
            .service(fs::Files::new("/static", "static"))
    })
    .bind(bind_address)?
    .run()
    .await
}
