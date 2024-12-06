use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use std::env;

use sea_orm::{DatabaseConnection, EntityTrait};

mod db;
use db::connect_to_db;

use entity::products;

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
    cfg.service(get_products);
    cfg.service(get_product);
    
}

#[get("/products")]
async fn get_products(db: web::Data<DatabaseConnection>) -> impl Responder {
    match products::Entity::find()
        .all(db.get_ref())
        .await {
            Ok(products) => HttpResponse::Ok().json(products),
            Err(err) => {
                eprintln!("Error fetching products: {:?}", err);
                HttpResponse::InternalServerError().body("Error fetching products")
            }
        }
}

#[get("/products/{id}")]
async fn get_product(product_id: web::Path<i32>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let id = product_id.into_inner();
    if id <= 0 {
        return HttpResponse::BadRequest().body("Invalid Product ID");
    }

    match products::Entity::find_by_id(id)
        .one(db.get_ref())
        .await {
            Ok(products) => HttpResponse::Ok().json(products),
            Err(err) => {
                eprintln!("Error fetching products: {:?}", err);
                HttpResponse::InternalServerError().body("Error fetching products")
            }
        }    
}