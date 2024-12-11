use actix_web::{post, web, HttpResponse};
use entity::{carts, order_items, order_status::OrderStatus, orders, products};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};

use crate::{api_error::APIError, auth_utils::Claims};
use sea_orm::ActiveValue::Set;

#[post("checkout")]
async fn checkout(
    db: web::Data<DatabaseConnection>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, APIError> {
    let customer_id: i64 = claims.sub.into();

    // Start a transaction
    let txn = db.begin().await.map_err(|e| APIError::DatabaseError(e))?;

    let cart_items_with_products = carts::Entity::find()
        .filter(carts::Column::CustomerId.eq(customer_id))
        .find_with_related(products::Entity)
        .all(&txn)
        .await
        .map_err(|e| APIError::DatabaseError(e))?;

    if cart_items_with_products.is_empty() {
        return Ok(HttpResponse::BadRequest().json({
            serde_json::json!({ "error": "Cart is empty. Cannot proceed with checkout." })
        }));
    }

    let new_order = orders::ActiveModel {
        customer_id: Set(customer_id),
        status: Set(OrderStatus::Pending.into()),
        ..Default::default()
    };

    let order = new_order
        .insert(&txn)
        .await
        .map_err(|e| APIError::DatabaseError(e))?;

    for (cart_item, products) in cart_items_with_products {
        if let Some(product) = products.first() {
            let new_order_item = order_items::ActiveModel {
                order_id: Set(order.id),
                product_id: Set(cart_item.product_id),
                unit_price: Set(product.price),
                amount: Set(cart_item.amount),
                ..Default::default()
            };

            new_order_item
                .insert(&txn)
                .await
                .map_err(|e| APIError::DatabaseError(e))?;
        } else {
            return Ok(HttpResponse::BadRequest().json({
                serde_json::json!({ "error": format!("Product not found for cart item ID {}", cart_item.id) })
            }));
        }
    }

    // Clear the customer's cart
    carts::Entity::delete_many()
        .filter(carts::Column::CustomerId.eq(customer_id))
        .exec(&txn)
        .await
        .map_err(|e| APIError::DatabaseError(e))?;

    // Commit the transaction
    txn.commit().await.map_err(|e| APIError::DatabaseError(e))?;

    // Return the order details
    Ok(HttpResponse::Ok().json({
        serde_json::json!({
            "message": "Checkout successful",
            "order_id": order.id
        })
    }))
}
