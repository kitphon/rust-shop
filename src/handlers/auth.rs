use actix_web::{post, web, HttpResponse};
use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::customers;
use crate::auth_utils::generate_jwt;
use crate::api_error::APIError;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub email: String,
}

#[post("/customer/register")]
pub async fn register(db: web::Data<DatabaseConnection>, form: web::Json<RegisterRequest>) -> Result<HttpResponse, APIError> {
    let password_hash = hash(&form.password, 12)
        .map_err(|_: bcrypt::BcryptError| APIError::InternalServerError)
        .expect("Failed to hash password");

    let new_user = customers::ActiveModel {
        email: sea_orm::ActiveValue::Set(form.email.clone()),
        name: sea_orm::ActiveValue::Set(form.name.clone()),
        password_hash: sea_orm::ActiveValue::Set(password_hash),
        ..Default::default()
    };

    match new_user.insert(db.get_ref()).await {
        Ok(user) => Ok(
            HttpResponse::Ok().json(RegisterResponse {
                id: user.id,
                email: user.email,
            }
        )),
        Err(e) => Err(APIError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String
}

#[post("/customer/login")]
pub async fn login(db: web::Data<sea_orm::DatabaseConnection>, form: web::Json<LoginRequest>) -> Result<HttpResponse, APIError> {
    let user = customers::Entity::find()
        .filter(customers::Column::Email.eq(&form.email))
        .one(db.get_ref())
        .await;

    match user {
        Ok(Some(user)) => {
            if verify(&form.password, &user.password_hash).unwrap_or(false) {
                let token = generate_jwt(&user.id, &user.email, &user.name);
                Ok(
                    HttpResponse::Ok().json(LoginResponse {
                        message: "Login successful".to_string(),
                        token
                    }
                ))
            } else {
                Err(APIError::AuthenticationError("Invalid credentials".to_string()))
            }
        }
        Ok(None) => Err(APIError::AuthenticationError("Invalid credentials".to_string())),
        Err(e) => Err(APIError::DatabaseError(e.to_string()))
    }
}
