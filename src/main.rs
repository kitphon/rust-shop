use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use std::env;

mod db;
use db::connect_to_db;
mod handlers; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = connect_to_db(&database_url).await;
    let _ = Migrator::up(&db, None).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::products::get_products);
    cfg.service(handlers::products::get_product);
    cfg.service(handlers::auth::register);
    cfg.service(handlers::auth::login);
}