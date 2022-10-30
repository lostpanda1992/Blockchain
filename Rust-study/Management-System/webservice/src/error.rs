use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

//自定义错误类型枚举
#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

// 传递给用户的错误类型
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

//将具体的错误类型转换成字符串消息
impl MyError{
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occured:{:?}", msg);
                "Database errror".into()
            },
            MyError::ActixError(msg) => {
                println!("Server error occured:{:?}", msg);
                "Internal server errror".into()
            },
            MyError::NotFound(msg) => {
                println!("Not found error occured:{:?}", msg);
                msg.into()
            },
            MyError::InvalidInput(msg) => {
                println!("Invalid paramters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

// 将错误类型转换成具体的http响应
// 为MyError实现error::ResponseError(actix_web自带的trait)
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode:: INTERNAL_SERVER_ERROR,
            MyError::NotFound(msg) => StatusCode:: NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

//当MyError实现error::ResponseError这个trait后，需要两个trait（debug display）
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

//将以下两种错误类型转换成MyError错误类型
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}