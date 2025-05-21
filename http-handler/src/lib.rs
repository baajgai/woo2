use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use std::fmt;

pub mod constant;
use constant::Woo2Err;

#[derive(Serialize, FromRow, Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub user_mobile: String,
    pub brand: String,
    pub model: String,
    pub year: String,
    pub mileage: f32,
    pub post_date: String,
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User(mobile: {})", self.user_mobile)
    }
}

#[derive(Serialize)]
pub struct ResponseVehicleWrapper {
    #[serde(rename = "VehicleList_Rec")]
    pub vehicle_list: Vec<Vehicle>,
}

#[derive(Serialize, FromRow, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub mobile: String,
}

#[derive(Serialize)]
pub struct ResponseWrapper {
    #[serde(rename = "UserList_Rec")]
    pub user_list: Vec<User>,
}

///
/// Vehicle create, vehicle list
///
pub async fn get_my_vehicles(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
    let user_mobile = req.headers().get("Param").and_then(|val| val.to_str().ok());

    if let Some(mobile) = user_mobile {
        println!("User mobile received {:?}", mobile);
    } else {
        println!("User mobile not set!!!");
        HttpResponse::InternalServerError().body(Woo2Err::ErrorUserNotFound.as_str());
    }

    let vehicle_result = sqlx::query_as::<_, Vehicle>(
        "SELECT id, brand, model, year, mileage, post_date FROM \"vehicle\" WHERE user_mobile = $1",
    )
    .bind(&user_mobile)
    .fetch_all(db_pool.get_ref())
    .await;

    match vehicle_result {
        Ok(vehicles) => {
            let wrapped = ResponseVehicleWrapper {
                vehicle_list: vehicles,
            };
            HttpResponse::Ok().json(wrapped)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body(Woo2Err::ErrorDbFetch.as_str())
        }
    }
}

pub async fn get_vehicles(db_pool: web::Data<PgPool>) -> impl Responder {
    let vehicle_result = sqlx::query_as::<_, Vehicle>(
        "SELECT id, brand, model, year, mileage, post_date FROM \"vehicle\"",
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match vehicle_result {
        Ok(vehicles) => {
            let wrapped = ResponseVehicleWrapper {
                vehicle_list: vehicles,
            };
            HttpResponse::Ok().json(wrapped)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body(Woo2Err::ErrorDbFetch.as_str())
        }
    }
}
pub async fn add_vehicle(
    db_pool: web::Data<PgPool>,
    new_vehicle: web::Json<Vehicle>,
) -> impl Responder {
    let result = sqlx::query(
        "INSERT INTO \"vehicle\" (brand, model, mileage, year, post_date, user_mobile) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&new_vehicle.brand)
    .bind(&new_vehicle.model)
    .bind(&new_vehicle.mileage)
    .bind(&new_vehicle.year)
    .bind(&new_vehicle.post_date)
    .bind(&new_vehicle.user_mobile)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(res) => HttpResponse::Created().body(format!("Rows affected: {}", res.rows_affected())),
        Err(e) => {
            println!("Error db {}", e);
            HttpResponse::InternalServerError().body(Woo2Err::ErrorDbAddValue.as_str())
        }
    }
}
// User create, user list
pub async fn add_user(
    req: HttpRequest,
    db_pool: web::Data<PgPool>,
    new_user: web::Json<User>,
) -> impl Responder {
    let session_id = req
        .headers()
        .get("Session-Id")
        .and_then(|val| val.to_str().ok());

    if let Some(session_id) = session_id {
        println!("Session-Id received: {}", session_id);
        // You can now use `session_id` in your logic (e.g., lookup session, auth check, etc.)
    } else {
        println!("No Session-Id header provided");
        HttpResponse::InternalServerError().body(Woo2Err::ErrorInvalidSession.as_str());
    }

    let result = sqlx::query("INSERT INTO \"user\" (name, mobile) VALUES ($1, $2)")
        .bind(&new_user.name)
        .bind(&new_user.mobile)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(res) => HttpResponse::Created().json(res.rows_affected()),
        Err(e) => {
            println!("Insert new user error {}", e);
            HttpResponse::InternalServerError().body(Woo2Err::ErrorDbAddValue.as_str())
        }
    }
}

pub async fn get_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let users_result = sqlx::query_as::<_, User>("SELECT id, name, mobile FROM \"user\"")
        .fetch_all(db_pool.get_ref())
        .await;

    match users_result {
        Ok(users) => {
            let wrapped = ResponseWrapper { user_list: users };
            HttpResponse::Ok().json(wrapped)
        }
        Err(e) => {
            println!("DB error: {}", e);
            HttpResponse::InternalServerError().body(Woo2Err::ErrorDbFetch.as_str())
        }
    }
}
