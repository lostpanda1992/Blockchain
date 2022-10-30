use actix_web::web;
use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use crate::errors::MyError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Teacher{
    pub id: i32, //serial 主键
    pub name: String,
    pub picture_url: String,
    pub profile:String,
}

//新增老师
#[derive(Debug, Deserialize, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile:String,
}

//修改老师信息
#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile:Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    type Error = MyError;

    fn from(new_teacher: web::Json<CreateTeacher>) -> Result<Self, Self::Error> {
        Ok(CreateTeacher{
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        })
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    type Error = MyError;

    fn from(update_teacher: web::Json<UpdateTeacher>) -> Result<Self, Self::Error> {
        Ok(UpdateTeacher{
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        })
    }
}
