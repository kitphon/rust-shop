use actix_web::{post, web, Responder};
use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::api_error::APIError;
use crate::auth_utils::generate_jwt;
use entity::customers;

use sea_orm::ActiveValue::Set;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "invalid"))]
    pub email: String,
    #[validate(length(min = 1, max = 255, message = "must be 1 - 255 characters long"))]
    pub name: String,
    #[validate(length(min = 10, message = "must be at least 10 characters long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub email: String,
}

#[post("/customer/register")]
pub async fn register(
    db: web::Data<DatabaseConnection>,
    form: web::Json<RegisterRequest>,
) -> Result<impl Responder, APIError> {
    match form.validate() {
        Ok(_) => (),
        Err(e) => return Err(APIError::ValidationError(e.to_string())),
    };

    let exist_customer = customers::Entity::find().one(db.get_ref()).await?;
    match exist_customer {
        Some(_) => {
            return Err(APIError::BadRequestError(
                "Email already exist.".to_owned(),
            ))
        }
        None => {}
    }

    let password_hash = hash(&form.password, 12)
        .map_err(|_: bcrypt::BcryptError| APIError::InternalServerError)
        .expect("Failed to hash password");

    let new_user = customers::ActiveModel {
        created_at: Set(chrono::Utc::now().fixed_offset()),
        email: Set(form.email.clone()),
        name: Set(form.name.clone()),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    let user: customers::Model = new_user.insert(db.get_ref()).await?;
    Ok(web::Json(RegisterResponse {
        id: user.id,
        email: user.email,
    }))
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "must be at least 1 character long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

#[post("/customer/login")]
pub async fn login(
    db: web::Data<sea_orm::DatabaseConnection>,
    form: web::Json<LoginRequest>,
) -> Result<impl Responder, APIError> {
    match form.validate() {
        Ok(_) => (),
        Err(e) => return Err(APIError::ValidationError(e.to_string())),
    };

    let user = customers::Entity::find()
        .filter(customers::Column::Email.eq(&form.email))
        .one(db.get_ref())
        .await?;

    match user {
        Some(user) => {
            if verify(&form.password, &user.password_hash).unwrap_or(false) {
                let token = generate_jwt(&user.id, &user.email, &user.name);
                Ok(web::Json(LoginResponse {
                    message: "Login successful".to_string(),
                    token,
                }))
            } else {
                Err(APIError::AuthenticationError(
                    "Invalid credentials".to_string(),
                ))
            }
        }
        None => Err(APIError::AuthenticationError(
            "Invalid credentials".to_string(),
        )),
    }
}
