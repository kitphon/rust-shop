use actix_web::{delete, post, web, HttpResponse};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use validator::Validate;

use crate::api_error::APIError;
use crate::auth_utils::Claims; 
use entity::carts;

#[derive(Deserialize, Validate)]
struct CartRequest {
    pub product_id: i64,
    pub amount: i32
}

#[post("/carts/add")]
pub async fn add(
    db: web::Data<DatabaseConnection>, req: web::Json<CartRequest>, claims: web::ReqData<Claims>
) -> Result<HttpResponse, APIError> {
    match req.validate() {
        Ok(_) => (),
        Err(e) => return Err(APIError::ValidationError(e. to_string())),
    };

    let customer_id: i64 = claims.sub.into();
    let existing_record = carts::Entity::find()
        .filter(carts::Column::ProductId.eq(req.product_id))
        .filter(carts::Column::CustomerId.eq(customer_id))
        .one(db.get_ref())
        .await
        .map_err(|e| APIError::DatabaseError(e))?;

    if let Some(record) = existing_record {
        let updated_amount = record.amount + req.amount;
        let mut active_model: carts::ActiveModel = record.into();
        active_model.amount = sea_orm::ActiveValue::Set(updated_amount);

        active_model
            .update(db.get_ref())
            .await
            .map_err(|e| APIError::DatabaseError(e))?;

        return Ok(HttpResponse::Ok().finish());
    }

    let new_item = carts::ActiveModel {
        product_id: sea_orm::ActiveValue::set(req.product_id),
        amount: sea_orm::ActiveValue::set(req.amount),
        customer_id: sea_orm::ActiveValue::set(customer_id),
        ..Default::default()
    };

    match new_item.insert(db.get_ref()).await {
        Ok(_) => Ok(
            HttpResponse::Created().finish()
        ),
        Err(e) => Err(APIError::DatabaseError(e))
    }
}

#[delete("/carts/clear")]
async fn clear(db: web::Data<DatabaseConnection>, claims: web::ReqData<Claims>) -> Result<HttpResponse, APIError> {
    let customer_id: i64 = claims.sub.into();

    match carts::Entity::delete_many()
        .filter(carts::Column::CustomerId.eq(customer_id))
        .exec(db.get_ref())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json({
            serde_json::json!({ "message": "Cart cleared successfully" })
        })),
        Err(e) => Err(APIError::DatabaseError(e))
    }
}

#[delete("/carts/{product_id}")]
async fn remove_product(
    db: web::Data<DatabaseConnection>, product_id_path: web::Path<i32>, claims: web::ReqData<Claims>
) -> Result<HttpResponse, APIError> {
    let product_id = product_id_path.into_inner();
    if product_id <= 0 {
        return Err(
            APIError::ValidationError("Invalid Product ID.".to_owned())
        )
    }
    let customer_id: i64 = claims.sub.into();

    match carts::Entity::delete_many()
        .filter(carts::Column::ProductId.eq(product_id))
        .filter(carts::Column::CustomerId.eq(customer_id))
        .exec(db.get_ref())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json({
            serde_json::json!({ "message": "The product was removed from the cart successfully." })
        })),
        Err(e) => Err(APIError::DatabaseError(e))
    }
}