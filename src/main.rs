use actix_web::{middleware, web, App, HttpServer};
use api_error::APIError;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use std::env;
use db::connect_to_db;
use jwt_middleware::JwtMiddleware;

mod db;
mod handlers; 
mod auth_utils;
mod jwt_middleware;
mod api_error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = connect_to_db(&database_url).await;
    let _ = Migrator::up(&db, None).await;

    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port_string_env = env::var("PORT").unwrap_or(String::from("8080"));
    let port: u16;
    match port_string_env.parse::<u16>() {
        Ok(parsed_port) => port = parsed_port,
        Err(_) => port = 8080
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                APIError::ValidationError(err.to_string()).into()
            }))
            .configure(init)
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    let api_path = env::var("API_PATH").unwrap_or(String::from("/api"));
    cfg.service(handlers::auth::register);
    cfg.service(handlers::auth::login);
    cfg.service(
        web::scope(&api_path)
            .wrap(JwtMiddleware)
            .service(handlers::products::get_products)
            .service(handlers::products::get_product),
    );
}