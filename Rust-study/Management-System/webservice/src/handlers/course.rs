use crate::models::course::{CreateCourse, UpdateCourse};

use crate::state::AppState;
use crate::dbaccess::course::*;
use crate::error::MyError;
use actix_web::{web, HttpResponse};

//注入new_course app_state
// 增加新课程
pub async fn new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse,MyError> {
    //此处修改原因：改用数据库进行增删改查
    // println!("Received new course");
    // let course_count = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id==new_course.teacher_id)
    //     .collect::<Vec<Course>>()
    //     .len();

    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };

    // app_state.courses.lock().unwrap().push(new_course);

    post_new_course_db(&app_state.db, new_course.try_into()?)
    .await.map(|course| HttpResponse::Ok().json(course))
    
}

//获得某个老师对应的课程
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params:web::Path<(i32,)>, // xxxx/{teacher_id}
) -> Result<HttpResponse,MyError> {
    //此处修改原因：改用数据库进行增删改查
    // let teacher_id: usize = params.0;

    // let filtered_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id==teacher_id)
    //     .collect::<Vec<Course>>();

    // if filtered_courses.len() > 0 {
    //     HttpResponse::Ok().json(filtered_courses)
    // } else {
    //     HttpResponse::Ok().json("No course found for teacher".to_string())
    // }

    //查出teacher_id
    // let teacher_id = i32::try_from(params.0).unwrap();
    let (teacher_id,) = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
    .await.map(|courses| HttpResponse::Ok().json(courses))
    

}

//获得课程的详细情况
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params:web::Path<(i32, i32)>,
) -> Result<HttpResponse,MyError> {
    //此处修改原因：改用数据库进行增删改查
    // let (teacher_id, course_id) = params.0;

    // let selected_courses = app_state
    // .courses
    // .lock()
    // .unwrap()
    // .clone()
    // .into_iter()
    // .find(|x| x.teacher_id==teacher_id &&
    //     x.id == Some(course_id))
    // .ok_or("Course not found");

    // if let Ok(course) = selected_courses {
    //     HttpResponse::Ok().json(course)
    // } else {
    //     HttpResponse::Ok().json("Course not found".to_string())
    // }

    // let teacher_id = i32::try_from(params.0).unwrap();
    // let course_id = i32::try_from(params.1).unwrap();
    let (teacher_id, course_id) = params.into_inner();
    get_courses_details_db(&app_state.db, teacher_id, course_id)
    .await.map(|course| HttpResponse::Ok().json(course))
    
}

// 删除课程
pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

// 修改课程细节
pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.try_into()?)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use dotenv::dotenv;
    use std::env;
    use sqlx::postgres::PgPoolOptions;

    #[actix_rt::test]
    async fn post_course_test() {
        //添加读取环境变量相关代码
        dotenv().ok();
        //创建数据库连接池
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();
        //初始化app_state
        let app_state = web::Data::new(AppState{
            health_check_response:"I'm ok".to_string(),
            visit_count:Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });

        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: Some(3), //数据库中类型为：serial
            time:None,
        });

        // let app_state = web::Data::new(AppState{
        //     health_check_response:"".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses:Mutex::new(vec![]), 
        // });

        let resp = new_course(course,app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}