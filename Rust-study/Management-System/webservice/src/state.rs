use std::sync::Mutex;
use sqlx::postgres::PgPool;

//Mutex保证线程安全
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses:Mutex<Vec<Course>>,
    // 原先是用内存存储 即Vec 
    pub db:PgPool,
}