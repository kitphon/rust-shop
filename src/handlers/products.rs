use actix_web::{get, web, Responder};
use entity::products;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::api_error::APIError;

#[get("/products")]
async fn get_products(db: web::Data<DatabaseConnection>) -> Result<impl Responder, APIError> {
    Ok(web::Json(products::Entity::find().all(db.get_ref()).await?))
}

#[get("/products/{id}")]
async fn get_product(
    product_id: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder, APIError> {
    let id = product_id.into_inner();
    if id <= 0 {
        return Err(APIError::ValidationError("Invalid Product ID.".to_owned()));
    }

    let product = products::Entity::find_by_id(id).one(db.get_ref()).await?;
    Ok(web::Json(APIError::from_option(
        product,
        "No Product Found",
    )?))
}
