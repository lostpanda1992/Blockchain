use crate::error::MyError;

use actix_web::web::{self, Json};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Course{
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

//新增课程
#[derive(Debug, Deserialize, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

//修改课程
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// //将请求json转化为createcourse类型数据
// impl From<web::Json<CreateCourse>> for CreateCourse {
//     fn from(course: web::Json<CreateCourse>) -> Self {
//         CreateCourse{
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price,
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     }
// }

//另一种方式将请求json转化为createcourse类型数据
impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>) -> Result<Self,Self::Error> {
        Ok(CreateCourse{
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {

    fn from(course: web::Json<UpdateCourse>) -> Result<Self, Self::Error> {
        Ok(UpdateCourse{
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}