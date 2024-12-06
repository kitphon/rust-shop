use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};

use entity::customers;

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
pub async fn register(db: web::Data<DatabaseConnection>, form: web::Json<RegisterRequest>) -> impl Responder {
    let password_hash = hash(&form.password, 12).expect("Failed to hash password");

    let new_user = customers::ActiveModel {
        email: sea_orm::ActiveValue::Set(form.email.clone()),
        name: sea_orm::ActiveValue::Set(form.name.clone()),
        password_hash: sea_orm::ActiveValue::Set(password_hash),
        ..Default::default()
    };

    match new_user.insert(db.get_ref()).await {
        Ok(user) => HttpResponse::Ok().json(RegisterResponse {
            id: user.id,
            email: user.email,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
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
}

#[post("/customer/login")]
pub async fn login(db: web::Data<sea_orm::DatabaseConnection>, form: web::Json<LoginRequest>) -> impl Responder {
    let user = customers::Entity::find()
        .filter(customers::Column::Email.eq(&form.email))
        .one(db.get_ref())
        .await;

    match user {
        Ok(Some(user)) => {
            if verify(&form.password, &user.password_hash).unwrap_or(false) {
                HttpResponse::Ok().json(LoginResponse {
                    message: "Login successful".to_string(),
                })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Error logging in"),
    }
}
