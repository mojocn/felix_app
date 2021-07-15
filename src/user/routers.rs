use crate::user::User;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::SqlitePool;

#[get("/users")]
async fn find_all(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all todos from database"),
    }
}

#[get("/users/{id}")]
async fn find(id: web::Path<i64>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = User::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Todo not found"),
    }
}

#[post("/user")]
async fn create(arg: web::Json<User>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = User::create(arg.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Error trying to create new todo"),
    }
}

#[put("/user/{id}")]
async fn update(
    id: web::Path<i64>,
    arg: web::Json<User>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let result = User::update(id.into_inner(), arg.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

#[delete("/user/{id}")]
async fn delete(id: web::Path<i64>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = User::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("User not found")
            }
        }
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
