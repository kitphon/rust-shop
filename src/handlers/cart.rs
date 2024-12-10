use actix_web::{post, web, HttpResponse};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use validator::Validate;

use crate::api_error::APIError;
use entity::carts;

#[derive(Deserialize, Validate)]
struct CartRequest {
    pub product_id: i64,
    pub amount: i32,
    pub customer_id: i64
}

#[post("/carts/add")]
pub async fn add(db: web::Data<DatabaseConnection>, req: web::Json<CartRequest>) -> Result<HttpResponse, APIError> {
    match req.validate() {
        Ok(_) => (),
        Err(e) => return Err(APIError::ValidationError(e. to_string())),
    };

    let existing_record = carts::Entity::find()
        .filter(carts::Column::ProductId.eq(req.product_id))
        .filter(carts::Column::CustomerId.eq(req.customer_id))
        .one(db.get_ref())
        .await
        .map_err(|e| APIError::DatabaseError(e.to_string()))?;

    if let Some(record) = existing_record {
        let updated_amount = record.amount + req.amount;
        let mut active_model: carts::ActiveModel = record.into();
        active_model.amount = sea_orm::ActiveValue::Set(updated_amount);

        active_model
            .update(db.get_ref())
            .await
            .map_err(|e| APIError::DatabaseError(e.to_string()))?;

        return Ok(HttpResponse::Ok().finish());
    }

    let new_item = carts::ActiveModel {
        product_id: sea_orm::ActiveValue::set(req.product_id),
        amount: sea_orm::ActiveValue::set(req.amount),
        customer_id: sea_orm::ActiveValue::set(req.customer_id),
        ..Default::default()
    };

    match new_item.insert(db.get_ref()).await {
        Ok(_) => Ok(
            HttpResponse::Created().finish()
        ),
        Err(e) => Err(APIError::DatabaseError(e.to_string()))
    }
}