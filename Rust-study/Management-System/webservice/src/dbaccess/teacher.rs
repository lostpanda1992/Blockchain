use crate::error::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::postgres::PgPool;

//查找老师
pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>,MyError> {
    let rows = sqlx::query!("SELECT * FROM teacher")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
    .iter()
    .map(|r| Teacher {
        id: r.id,
        name: r.name.clone(),
        picture_url: r.picture_url.clone(),
        profile: r.profile.clone(),
    }).collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("NO teacher found".into())),
        _ => Ok(teachers)
    }
}

//获得老师具体信息
pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Teacher>,MyError> {
    let row = sqlx::query!(
        "SELECT * FROM teacher where id = $1",teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        })
        .map_err(|_err| MyError::NotFound("NO teacher found".into()))?;

    Ok(row)
}

//新增老师信息
pub async fn post_new_teacher_db(pool: &PgPool, new_teacher:CreateTeachers) -> Result<Teacher,MyError> {
    let row = sqlx::query!(
        "INSERT INTO teacher (name, picture_url, profile) VALUES ($1, $2,$3)
        RETURNING name, picture_url, profile",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile)
        .fetch_one(pool)
        .await?;

    Ok(Teacher { 
        id: row.id, 
        name: row.name, 
        picture_url: row.picture_url, 
        profile: row.profile
    })
}

//修改老师信息
pub async fn update_teacher_details_db(pool: &PgPool, teacher_id:i32, update_teacher:UpdateTeachers) -> Result<Teacher,MyError> {
    let row = sqlx::query!(
        "SELECT * FROM teacher where id = $1", teacher_id)
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("NO teacher found".into()))?;

    let temp = Teacher { 
        id: row.id, 
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name
        }, 
        picture_url: if let Some(pic) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url
        }, 
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile
        },
    };

    let update_row = sqlx::query!(
        "UPDATE teacher SET name = $1, picture_url = $2, profile = $3 WHERE  id = $4 RETURNING id, name, picture_url, profile",
        temp.name,
        temp.picture_url,
        temp.profile,
        teacher_id
    )
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name,
            picture_url: r.picture_url,
            profile: r.profile,
        })
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    
    Ok(())
}

//删除老师
pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("DELETE FROM teacher WHERE id = {}",teacher_id))
    .execute(pool)
    .await
    .map_err(|_err| MyError::DBError("NO teacher found".into()))?;
    
    Ok(format!("Deleted {:?} record", row))
}


