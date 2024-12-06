use actix_web::{get, web, HttpResponse, Responder};
use entity::products;
use sea_orm::{DatabaseConnection, EntityTrait};

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