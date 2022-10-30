use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::sync::Mutex;
use std::{io, env};
use dotenv::dotenv;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../error.rs"]
mod error;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    //添加读取环境变量相关代码
    dotenv().ok();

    //创建数据库连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState{
        health_check_response:"I'm ok".to_string(),
        visit_count:Mutex::new(0),
        // courses:Mutex::new(vec![]),
        db: db_pool,
    });
    //添加路由注册
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Player provide valid Json input".to_string()).into()
            }))
            .wrap(cors)
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };
    
    println!("Service is running");
    HttpServer::new(app).bind("127.0.0.1:3001")?.run().await
}


